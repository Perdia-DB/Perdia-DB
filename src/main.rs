use tokio::{net::{TcpListener}, signal};
use dotenv::{self};

mod lexer;
mod data;
mod query;
mod backup;
mod util;
mod server;
mod crypto;

#[tokio::main]
async fn main() {
    plog!("Started");
    match dotenv::dotenv() {
        Ok(_) => plog!("Loaded .env file!"),
        Err(_) => perr!("Failed to load .env file!")
    }

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    plog!("Running at addr: {}", listener.local_addr().unwrap());

    server::run(listener, signal::ctrl_c()).await;
    
    plog!("Shutdown complete!")
}


