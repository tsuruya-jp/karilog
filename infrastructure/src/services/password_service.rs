use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use domain::value_objects::{Password, PasswordHash as DomainPasswordHash};
use shared::error::AppError;

/// パスワードハッシング・検証サービス
#[derive(Clone)]
pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }

    /// パスワードをハッシュ化
    pub fn hash_password(&self, password: &Password) -> Result<DomainPasswordHash, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_str().as_bytes(), &salt)
            .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?
            .to_string();

        Ok(DomainPasswordHash::new(password_hash))
    }

    /// パスワードを検証
    pub fn verify_password(
        &self,
        password: &str,
        password_hash: &DomainPasswordHash,
    ) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(password_hash.as_str()).map_err(|e| {
            AppError::InternalServerError(format!("Failed to parse password hash: {}", e))
        })?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::Password;

    #[test]
    fn test_hash_and_verify_password() {
        let service = PasswordService::new();
        let password = Password::new("TestPassword123!".to_string()).unwrap();

        let hash = service.hash_password(&password).unwrap();

        // 正しいパスワードで検証
        assert!(service.verify_password(password.as_str(), &hash).unwrap());

        // 間違ったパスワードで検証
        assert!(!service.verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_hash_produces_different_results() {
        let service = PasswordService::new();
        let password = Password::new("TestPassword123!".to_string()).unwrap();

        let hash1 = service.hash_password(&password).unwrap();
        let hash2 = service.hash_password(&password).unwrap();

        // 同じパスワードでも異なるハッシュが生成される（ソルトが異なるため）
        assert_ne!(hash1.as_str(), hash2.as_str());

        // しかし、両方とも元のパスワードで検証できる
        assert!(service.verify_password(password.as_str(), &hash1).unwrap());
        assert!(service.verify_password(password.as_str(), &hash2).unwrap());
    }
}
