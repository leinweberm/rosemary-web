use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn password_verify(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
	verify(&password, &hashed)
}

pub async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
	hash(&password, DEFAULT_COST)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_password() {
        let password = "my_secure_password";
        let hashed = hash_password(password).await.unwrap();
        assert!(!hashed.is_empty());

        let is_valid_hash = verify(password, &hashed).unwrap();
        assert!(is_valid_hash);
    }

    #[tokio::test]
    async fn test_password_verify() {
        let password = "my_secure_password";
        let hashed = hash_password(password).await.unwrap();
        let is_valid = password_verify(password, &hashed).await.unwrap();
        assert!(is_valid);

        let is_invalid = password_verify("wrong_password", &hashed).await.unwrap();
        assert!(!is_invalid);
    }
}