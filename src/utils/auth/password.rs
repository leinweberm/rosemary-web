use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn password_verify(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
	verify(&password, &hashed)
}

pub async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
	hash(&password, DEFAULT_COST)
}