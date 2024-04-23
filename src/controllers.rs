use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::constants::INTERNAL_SERVER_ERROR;
use crate::constants::NOT_FOUND;
use crate::constants::OK_RESPONSE;
use crate::DB_URL;

pub fn handle_post_request(request: &str) -> (String, String) {
    match (
        crate::utils::get_book_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(book), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO books (name, email) VALUES ($1, $2)",
                    &[&book.author, &book.title],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Book created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn handle_get_request(request: &str) -> (String, String) {
    match (
        crate::utils::get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM books WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let book = crate::models::Book {
                        id: row.get(0),
                        author: row.get(1),
                        title: row.get(2),
                    };

                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&book).unwrap(),
                    )
                }
                _ => (NOT_FOUND.to_string(), "Book not found".to_string()),
            }
        }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut books: Vec<crate::models::Book> = Vec::new();

            for row in client.query("SELECT * FROM books", &[]).unwrap() {
                books.push(crate::models::Book {
                    id: row.get(0),
                    author: row.get(1),
                    title: row.get(2),
                });
            }

            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&books).unwrap(),
            )
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn handle_put_request(request: &str) -> (String, String) {
    match (
        crate::utils::get_id(&request).parse::<i32>(),
        crate::utils::get_book_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(book), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE books SET author = $1, title = $2 WHERE id = $3",
                    &[&book.author, &book.title, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Book updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn handle_delete_request(request: &str) -> (String, String) {
    match (
        crate::utils::get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client
                .execute("DELETE FROM books WHERE id = $1", &[&id])
                .unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "Book not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Book deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}
