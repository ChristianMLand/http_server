#![allow(dead_code)]

use website_handler::WebsiteHandler;
use server::Server;
use std::env;

mod website_handler;
mod server;
mod http;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler::new(public_path));
}