use crate::value_objects::{Email, PasswordHash, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ユーザーエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub name: Option<String>,
    pub email_verified: bool,
    pub email_verification_token: Option<String>,
    pub email_verification_expires_at: Option<DateTime<Utc>>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    /// 新しいユーザーを作成
    pub fn new(email: Email, password_hash: PasswordHash, name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            password_hash,
            name,
            email_verified: false,
            email_verification_token: None,
            email_verification_expires_at: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            failed_login_attempts: 0,
            locked_until: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// アカウントがロックされているか確認
    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            Utc::now() < locked_until
        } else {
            false
        }
    }

    /// メール確認トークンが有効か確認
    pub fn is_email_verification_token_valid(&self, token: &str) -> bool {
        if let (Some(stored_token), Some(expires_at)) = (
            &self.email_verification_token,
            &self.email_verification_expires_at,
        ) {
            stored_token == token && Utc::now() < *expires_at
        } else {
            false
        }
    }

    /// パスワードリセットトークンが有効か確認
    pub fn is_password_reset_token_valid(&self, token: &str) -> bool {
        if let (Some(stored_token), Some(expires_at)) =
            (&self.password_reset_token, &self.password_reset_expires_at)
        {
            stored_token == token && Utc::now() < *expires_at
        } else {
            false
        }
    }

    /// ログイン失敗をカウント
    pub fn increment_failed_login_attempts(&mut self) {
        self.failed_login_attempts += 1;
        self.updated_at = Utc::now();

        // 5回失敗したら30分ロック
        if self.failed_login_attempts >= 5 {
            self.locked_until = Some(Utc::now() + chrono::Duration::minutes(30));
        }
    }

    /// ログイン失敗カウントをリセット
    pub fn reset_failed_login_attempts(&mut self) {
        self.failed_login_attempts = 0;
        self.locked_until = None;
        self.updated_at = Utc::now();
    }

    /// メール確認トークンを設定
    pub fn set_email_verification_token(&mut self, token: String, expires_in_hours: i64) {
        self.email_verification_token = Some(token);
        self.email_verification_expires_at =
            Some(Utc::now() + chrono::Duration::hours(expires_in_hours));
        self.updated_at = Utc::now();
    }

    /// メール確認を完了
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.email_verification_token = None;
        self.email_verification_expires_at = None;
        self.updated_at = Utc::now();
    }

    /// パスワードリセットトークンを設定
    pub fn set_password_reset_token(&mut self, token: String, expires_in_hours: i64) {
        self.password_reset_token = Some(token);
        self.password_reset_expires_at =
            Some(Utc::now() + chrono::Duration::hours(expires_in_hours));
        self.updated_at = Utc::now();
    }

    /// パスワードをリセット
    pub fn reset_password(&mut self, new_password_hash: PasswordHash) {
        self.password_hash = new_password_hash;
        self.password_reset_token = None;
        self.password_reset_expires_at = None;
        self.updated_at = Utc::now();
    }

    /// ユーザー名を更新
    pub fn update_name(&mut self, name: Option<String>) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// 論理削除
    pub fn delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::{Email, PasswordHash};

    fn create_test_user() -> User {
        User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            PasswordHash::new("hashed_password".to_string()),
            Some("Test User".to_string()),
        )
    }

    #[test]
    fn test_new_user() {
        let user = create_test_user();
        assert_eq!(user.email.as_str(), "test@example.com");
        assert!(!user.email_verified);
        assert_eq!(user.failed_login_attempts, 0);
        assert!(!user.is_locked());
    }

    #[test]
    fn test_increment_failed_login_attempts() {
        let mut user = create_test_user();

        for _ in 0..4 {
            user.increment_failed_login_attempts();
            assert!(!user.is_locked());
        }

        user.increment_failed_login_attempts();
        assert_eq!(user.failed_login_attempts, 5);
        assert!(user.is_locked());
    }

    #[test]
    fn test_reset_failed_login_attempts() {
        let mut user = create_test_user();
        user.failed_login_attempts = 5;
        user.locked_until = Some(Utc::now() + chrono::Duration::minutes(30));

        user.reset_failed_login_attempts();
        assert_eq!(user.failed_login_attempts, 0);
        assert!(!user.is_locked());
    }

    #[test]
    fn test_verify_email() {
        let mut user = create_test_user();
        user.set_email_verification_token("token123".to_string(), 24);

        assert!(!user.email_verified);
        assert!(user.email_verification_token.is_some());

        user.verify_email();
        assert!(user.email_verified);
        assert!(user.email_verification_token.is_none());
    }
}
