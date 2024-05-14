#![allow(dead_code)]
use std::io;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;

pub async fn write_bytes(data: &[u8], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path).await?;
    file.write_all(data).await?;
    Ok(())
}

pub async fn append_bytes(data: &[u8], file_path: &str, create_file: bool) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(create_file)
        .open(file_path)
        .await?;

    file.write_all(data).await?;
    Ok(())
}