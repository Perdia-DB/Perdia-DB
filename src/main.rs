#![feature(path_try_exists)]
#[cfg(target_os = "windows")]
use tokio::{net::{TcpListener}, signal};
#[cfg(target_os = "linux")]
use tokio::{net::{TcpListener}, signal::unix::{signal, SignalKind}};

mod lexer;
mod data;
mod query;
mod backup;
mod util;
mod server;
mod crypto;
mod ast;
mod error;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    plog!("Running at addr: {}", listener.local_addr().unwrap());
    dotenv::dotenv().unwrap();
    #[cfg(target_os = "windows")]
    server::run(listener, signal::ctrl_c()).await;
    #[cfg(target_os = "linux")]
    server::run(listener, signal(SignalKind::terminate()).unwrap().recv()).await;
    
    plog!("Shutdown complete!")
}


