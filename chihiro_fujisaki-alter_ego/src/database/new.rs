use super::Database;
use crate::aliases::error_result::Hope;
use tokio_postgres::NoTls;

impl Database
{
    pub async fn new(url: &str) -> Hope<Self>
    {
        let (client, socket_stream_connection) = tokio_postgres::connect(&url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = socket_stream_connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let client_database = Self { client };

        Ok(client_database)
    }
}
