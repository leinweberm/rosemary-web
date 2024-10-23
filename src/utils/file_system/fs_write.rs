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
        .append(true)
        .create(create_file)
        .open(file_path)
        .await?;

    file.write_all(data).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
	use crate::utils::file_system::fs_read;
	use super::*;
	use tokio::fs;

	#[tokio::test]
	async fn test_file_lifecycle() {
		let file_path = String::from("testFile.txt");
		let file_content = String::from("Hello World!");
		let string_content = file_content.clone();
		let content: Vec<u8> = file_content.into_bytes();

		write_bytes(&content, &file_path).await.unwrap();

		let exists = fs_read::file_exists(&file_path).await;
		assert_eq!(exists, true);

		let size = fs_read::get_file_size(&file_path).await;
		let has_valid_size = size as u64 > 0u64;
		assert_eq!(has_valid_size, true);

		let read_file_string = fs_read::read_file_to_string(&file_path).await;
		assert_eq!(&read_file_string, &string_content);

		let read_file_bytes = fs_read::read_file(&file_path).await;
		assert_eq!(&read_file_bytes, &content);

		let removed = match fs::remove_file(&file_path).await {
			Ok(_) => true,
			Err(_) => false,
		};
		assert_eq!(removed, true);
	}
}