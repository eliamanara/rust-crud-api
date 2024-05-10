use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod constants;
mod controllers;
mod database;
mod models;
mod utils;

#[macro_use]
extern crate serde_derive;

const DB_URL: &str = env!("DATABASE_URL");
const DATABASE_USERNAME: &str = env!("DATABASE_USERNAME");
const DATABASE_PASSWORD: &str = env!("DATABASE_PASSWORD");

fn main() {
    //set database
    if let Err(e) = database::set_database(DB_URL) {
        println!("Error: {}", e);
        return;
    }

    if let Err(e) = database::create_admin_user(DB_URL, DATABASE_USERNAME, DATABASE_PASSWORD) {
        println!("Error creating admin user: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /books") => controllers::handle_post_request(r),
                r if r.starts_with("GET /books/") => controllers::handle_get_request(r),
                r if r.starts_with("GET /books") => controllers::handle_get_all_request(r),
                r if r.starts_with("PUT /books/") => controllers::handle_put_request(r),
                r if r.starts_with("DELETE /books/") => controllers::handle_delete_request(r),
                _ => (
                    constants::NOT_FOUND.to_string(),
                    "404 Not Found".to_string(),
                ),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
