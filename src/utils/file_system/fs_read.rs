#![allow(dead_code)]
use tokio::fs;

pub async fn file_exists(file_path: &str) -> bool {
    match fs::metadata(file_path).await {
        Ok(_) => true,
        Err(error) => {
            error!(target: "fs", "Failed to read file metadata {}", error);
            false
        }
    }
}

pub async fn get_file_size(file_path: &str) -> u64 {
    match fs::metadata(file_path).await {
        Ok(meta) => meta.len(),
        Err(error) => {
            error!(target: "fs", "Failed to read file metadata {}", error);
            0
        }
    }
}

pub async fn read_file(file_path: &str) -> Vec<u8> {
    fs::read(file_path).await.unwrap_or_else(|error| {
        error!(target: "fs", "Failed to read file {}", error);
        Vec::new()
    })
}

pub async fn read_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).await.unwrap_or_else(|error| {
        error!(target: "fs", "Failed to read file into string {}", error);
        String::from("")
    })
}
