#![feature(path_try_exists)]
#[cfg(target_os = "windows")]
use tokio::{net::{TcpListener}, signal};
#[cfg(target_os = "unix")]
use tokio::{net::{TcpListener}, signal::unix::{signal, SignalKind}};

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

    let listener = TcpListener::bind("[::]:3000").await.unwrap();
    plog!("Running at addr: {}", listener.local_addr().unwrap());

    #[cfg(target_os = "windows")]
    server::run(listener, signal::ctrl_c()).await;
    #[cfg(target_os = "unix")]
    server::run(listener, signal(SignalKind::quit()).unwrap()).await;
    
    plog!("Shutdown complete!")
}


