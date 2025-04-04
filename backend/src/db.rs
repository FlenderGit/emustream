use mongodb::{Client, Database, options::ClientOptions};

pub async fn get_database(
    database_url: &str,
    database_name: &str,
) -> Result<Database, DatabaseError> {
    let mut client_options = ClientOptions::parse(database_url).await.map_err(
        |e| DatabaseError::ConnectionError(format!("Failed to parse client options: {}", e)),
    )?;
    client_options.app_name = Some("emustream".to_string());
    let client = Client::with_options(client_options).unwrap();
    Ok(client.database(database_name))
}

pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "Query error: {}", msg),
        }
    }
    
}

use std::io;

impl From<DatabaseError> for io::Error {
    fn from(err: DatabaseError) -> io::Error {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Database error: {}", err),
        )
    }
}
