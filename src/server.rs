use std::future::Future;

use serde::Serialize;
use tokio::{net::{TcpListener, TcpStream}, io::{Interest, AsyncWriteExt}};

use crate::{lexer, query::{self, error::RequestError}, BUFFER_SIZE, perr, plog, backup::SaveWorker};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u8,
    description: &'static str,
}

impl From<RequestError> for ErrorResponse {
    fn from(err: RequestError) -> Self {
        match err {
            RequestError::TemplateNonExistent => ErrorResponse {
                code: 100,
                description: "You have tried to create an instance of a template that doesn't currently exist.",
            },
            RequestError::TemplateAlreadyExists => ErrorResponse {
                code: 101,
                description: "You have tried to create a template that already exists.",
            },
            RequestError::InstanceNonExistent => ErrorResponse {
                code: 200,
                description: "You have tried to query a instance that doesn't exist.",
            },
            RequestError::InstanceAlreadyExists => ErrorResponse {
                code: 201,
                description: "You have tried to create a instance that already exists.",
            },
            RequestError::SyntaxError => ErrorResponse {
                code: 10,
                description: "General syntax error in source.",
            },
            RequestError::SerializationError => ErrorResponse {
                code: 1,
                description: "Internal db error, failed to serialize to json string.",
            },
        }
    }
}


struct Server {
    listener: TcpListener,
}

impl Server {
    // Process incoming request and query
    async fn process(stream: &mut TcpStream) -> Result<(), Error> {
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;
        
        if ready.is_readable() && ready.is_writable() {
            
            let mut buf = Vec::with_capacity(BUFFER_SIZE);
            let len = stream.try_read_buf(&mut buf)?;
            let source = String::from_utf8(buf.split_at(len).0.to_vec())?;
            let result = query::data(lexer::parse(&source));

            match result {
                Ok(value) => { stream.write(value.as_str().as_bytes()).await?; },
                Err(error) => { 
                    let response = serde_json::to_string(&ErrorResponse::from(error)).unwrap();
                    stream.write(response.as_str().as_bytes()).await?;
                },
            };
        }

        stream.shutdown().await?;
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            let (mut stream, addr) = self.listener.accept().await?;
            plog!("Query was issued from: {}", addr.ip());
            Server::process(&mut stream).await?;
        }
    }
}

pub async fn run(listener: TcpListener, shutdown: impl Future)  {

    let save_worker = SaveWorker::new();

    let mut server = Server {
        listener,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                save_worker.shutdown();
                perr!("Failed to accept: {}", err);
            }
        }
        _ = shutdown => {
            save_worker.shutdown();
            plog!("Shutting down server!")
        }
    }
}