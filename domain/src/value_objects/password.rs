use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password must be at least 8 characters long")]
    TooShort,
    #[error("Password must contain at least one uppercase letter")]
    NoUppercase,
    #[error("Password must contain at least one lowercase letter")]
    NoLowercase,
    #[error("Password must contain at least one digit")]
    NoDigit,
}

/// パスワード値オブジェクト（平文）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    value: String,
}

impl Password {
    /// 新しいPasswordインスタンスを作成
    pub fn new(value: String) -> Result<Self, PasswordError> {
        Self::validate_password(&value)?;
        Ok(Self { value })
    }

    /// パスワードのバリデーション
    fn validate_password(password: &str) -> Result<(), PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::TooShort);
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(PasswordError::NoUppercase);
        }

        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(PasswordError::NoLowercase);
        }

        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(PasswordError::NoDigit);
        }

        Ok(())
    }

    /// 文字列として取得
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// 所有権を持つStringとして取得
    pub fn into_string(self) -> String {
        self.value
    }
}

/// パスワードハッシュ値オブジェクト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHash {
    value: String,
}

impl PasswordHash {
    /// 新しいPasswordHashインスタンスを作成
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// 文字列として取得
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// 所有権を持つStringとして取得
    pub fn into_string(self) -> String {
        self.value
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = Password::new("SecurePass123".to_string());
        assert!(password.is_ok());
    }

    #[test]
    fn test_password_too_short() {
        let password = Password::new("Short1A".to_string());
        assert!(matches!(password, Err(PasswordError::TooShort)));
    }

    #[test]
    fn test_password_no_uppercase() {
        let password = Password::new("securepass123".to_string());
        assert!(matches!(password, Err(PasswordError::NoUppercase)));
    }

    #[test]
    fn test_password_no_lowercase() {
        let password = Password::new("SECUREPASS123".to_string());
        assert!(matches!(password, Err(PasswordError::NoLowercase)));
    }

    #[test]
    fn test_password_no_digit() {
        let password = Password::new("SecurePassword".to_string());
        assert!(matches!(password, Err(PasswordError::NoDigit)));
    }
}
