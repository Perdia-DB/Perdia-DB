use std::env;
use dotenv::dotenv;

use backup::SaveWorker;
use data::{TEMPLATES, template::Template};
use serde::Serialize;
use serde_json::*;
use tokio::{net::{TcpListener, TcpSocket, TcpStream}, io::{Interest, AsyncWriteExt}};

use crate::data::INSTANCES;
use query::error::RequestError;

mod lexer;
mod data;
mod query;
mod backup;

static BUFFER_SIZE: usize = 1048576;

#[tokio::main]
async fn main() {
    dotenv().ok(); // for testing

    // Init the background worker for disk-writes
    let save_worker = SaveWorker::new().init();

    // Set the webserver to listen to test-port
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Server loop to process incomming requests
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        process(&mut stream).await;
    }
    // Shutdown the background worker for disk-writes
    save_worker.shutdown();
}

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

// Process incoming request and query
async fn process(stream: &mut TcpStream) {
    let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.expect("Couldn't read stream.");
    
    if ready.is_readable() && ready.is_writable() {
        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        let len = stream.try_read_buf(&mut buf).expect("Failed to stream to buffer.");
        let source = String::from_utf8(buf.split_at(len).0.to_vec()).expect("Input is not UTF-8.");
        println!("{}", source);
        let result = query::data(lexer::parse(&source));
        match result {
            Ok(value) => { stream.write(value.as_str().as_bytes()).await.expect("Couldn't send result."); },
            Err(error) => { 
                let response = serde_json::to_string(&ErrorResponse::from(error)).unwrap();
                stream.write(response.as_str().as_bytes()).await.expect("Couldn't send result.");
            },
        };
    }

    stream.shutdown();
}