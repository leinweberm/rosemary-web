use std::path::Path;
use tokio::fs;

pub async fn remove_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    match fs::remove_file(&path).await {
        Ok(_) => true,
        Err(error) => {
            error!(target: "fs", "remove_file:error - {:?}", error);
            false
        }
    }
}
