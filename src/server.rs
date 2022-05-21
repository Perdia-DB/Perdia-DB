use std::future::Future;

use aes::{Aes256, cipher::{KeyInit, generic_array::GenericArray}, Aes128};
use serde::Serialize;
use tokio::{net::{TcpListener, TcpStream}, io::{Interest, AsyncWriteExt}};

use crate::{lexer, query::{self, error::RequestError}, perr, plog, backup::SaveWorker, crypto::Key, pwarn};

type Error = Box<dyn std::error::Error + Send + Sync>;
static BUFFER_SIZE: usize = 1048576;

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
    aes_key: Vec<u8>,
}

impl Server {
    // Process incoming request and query
    async fn process(&self, stream: &mut TcpStream) -> Result<(), Error> {
        (stream.readable().await?, stream.writable().await?);

        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        let len = stream.try_read_buf(&mut buf)?;
        let source = String::from_utf8(buf.split_at(len).0.to_vec())?;
        let result = query::data(lexer::parse(&source));
        self.send(stream, result).await?;

        stream.shutdown().await?;
        Ok(())
    }

    async fn send(&self, stream: &mut TcpStream, result: Result<String, RequestError>) -> Result<(), Error> {
        let output = match result {
            Ok(value) => value.into_bytes(),
            Err(error) => { 
                let response = serde_json::to_string(&ErrorResponse::from(error)).unwrap();
                response.into_bytes()
            },
        };
        stream.write(&self.encrypt(output)).await?;
        Ok(())
    }

    fn encrypt(&self, input: Vec<u8>) -> Vec<u8> {
        let key: Key = self.aes_key.clone().into();
        key.encrypt(input)
    }

    fn decrypt(&self, input: Vec<u8>) -> Vec<u8> {
        let key: Key = self.aes_key.clone().into();
        key.decrypt(input)
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            let (mut stream, addr) = self.listener.accept().await?;
            plog!("Query was issued from: {}", addr.ip());
            self.process(&mut stream).await?;
        }
    }
}

pub async fn run(listener: TcpListener, shutdown: impl Future)  {

    let save_worker = SaveWorker::new();

    let aes_key = match std::env::var("AES_KEY") {
        Ok(key) => key.into_bytes(),
        Err(_) => return perr!("Failed to read AES-Key, needed for encryption!"),
    };

    let mut server = Server {
        listener,
        aes_key,
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