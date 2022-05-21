use tokio::{net::{TcpListener}, signal};

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

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    plog!("Running at addr: {}", listener.local_addr().unwrap());

    server::run(listener, signal::ctrl_c()).await;
    
    plog!("Shutdown complete!")
}


