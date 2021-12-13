use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::Read;

pub trait Handler {
    fn handle_request(&mut self, request : &Request) -> Response;

    fn handle_bad_request(&mut self, error : &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address : String
}

impl Server {
    pub fn new(address : String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler : impl Handler) {
        println!("Listening on {}",self.address);

        let listener = TcpListener::bind(&self.address).unwrap();//pass reference to address
        
        loop {
            match listener.accept() {
                Ok((mut stream,sock_address)) => {
                    println!("OK established connection with: {}",sock_address);
                    let mut buffer = [0 ; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(error) => handler.handle_bad_request(&error)
                            };

                            if let Err(error) = response.send(&mut stream) {
                                println!("Failed to send response: {}", error);
                            }
                        },
                        Err(error) => println!("Failed to read from the connection: {}", error)
                    }
                },
                Err(error) => println!("Failed to establish a connection: {}", error)
            }
        }
    }
}