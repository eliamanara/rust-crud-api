use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn set_database(db_url: &str) -> Result<(), PostgresError> {
    // connect to database
    let mut client = Client::connect(db_url, NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR NOT NULL,
            password VARCHAR NOT NULL
        )",
    )?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS books (
            id SERIAL PRIMARY KEY,
            author VARCHAR NOT NULL,
            title VARCHAR NOT NULL
        )",
    )?;
    Ok(())
}

pub fn create_admin_user(
    db_url: &str,
    username: &str,
    password: &str,
) -> Result<(), PostgresError> {
    let mut client: Client = Client::connect(db_url, NoTls)?;
    let query =
        format!("INSERT INTO users (username, password) VALUES ('{username}', '{password}');");
    client.batch_execute(&query)?;
    Ok(())
}
