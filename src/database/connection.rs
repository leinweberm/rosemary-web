use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error};
use lazy_static::lazy_static;
use tokio::sync::OnceCell;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use crate::utils::file_system::fs_read;

lazy_static! {
    pub static ref CLIENT: OnceCell<Client> = OnceCell::const_new();
}

pub async fn init_connection() -> Result<(), Error> {
    dotenv().ok();

    let database_url = env::var("database_url")
        .expect("$database_url must be set");
    let cert_path = String::from("certs/root.crt");

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let check = fs_read::file_exists(&cert_path).await;
    if !check {
        panic!("CA cert file not found");
    }

    builder.set_ca_file(&cert_path).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(&database_url, connector).await?;


    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("connection error {}", e);
        }
    });

    let rows = client
        .query("SELECT 1 + 1", &[])
        .await?;

    let value: i64 = rows[0].get(0);
    assert_eq!(value, 2);

    CLIENT.set(client).expect("Failed to set client");

    Ok(())
}

pub async fn get_client() -> Result<&'static Client, std::io::Error> {
    CLIENT.get().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Client not"))
}
