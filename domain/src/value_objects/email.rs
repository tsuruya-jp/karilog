use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email format")]
    InvalidFormat,
}

/// メールアドレス値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    /// 新しいEmailインスタンスを作成
    pub fn new(value: String) -> Result<Self, EmailError> {
        let email = Self { value };
        email.validate().map_err(|_| EmailError::InvalidFormat)?;
        Ok(email)
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

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("test@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::new("invalid-email".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        assert_eq!(email.to_string(), "test@example.com");
    }
}
