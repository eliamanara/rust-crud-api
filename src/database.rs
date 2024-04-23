use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn set_database(db_url: &str) -> Result<(), PostgresError> {
    // connect to database
    let mut client = Client::connect(db_url, NoTls)?;

    // create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS books (
            id SERIAL PRIMARY KEY,
            author VARCHAR NOT NULL,
            title VARCHAR NOT NULL
        )",
    )?;
    Ok(())
}
