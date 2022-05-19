use std::env;
use dotenv::dotenv;

use backup::SaveWorker;
use data::{TEMPLATES, template::Template};
use serde::Serialize;
use serde_json::*;
use tokio::{net::{TcpListener, TcpSocket, TcpStream}, io::{Interest, AsyncWriteExt}, signal};

use crate::data::INSTANCES;
use query::error::RequestError;

mod lexer;
mod data;
mod query;
mod backup;
mod util;
mod server;


static BUFFER_SIZE: usize = 1048576;

#[tokio::main]
async fn main() {
    plog!("Started");
    dotenv().ok(); // for testing

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    server::run(listener, signal::ctrl_c()).await;
    
    plog!("Shutdown complete!")
}


