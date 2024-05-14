#![allow(dead_code)]
use tokio::fs;

pub async fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).await.is_ok()
}

pub async fn get_file_size(file_path: &str) -> u64 {
    if let Ok(metadata) = fs::metadata(file_path).await {
        if file_exists(file_path).await {
            metadata.len()
        } else {
            0
        }
    } else {
        0
    }
}