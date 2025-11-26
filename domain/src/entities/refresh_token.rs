use crate::value_objects::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// リフレッシュトークンエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: UserId,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl RefreshToken {
    /// 新しいリフレッシュトークンを作成
    pub fn new(user_id: UserId, token_hash: String, expires_in_days: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at: now + chrono::Duration::days(expires_in_days),
            created_at: now,
            revoked_at: None,
        }
    }

    /// トークンが有効か確認
    pub fn is_valid(&self) -> bool {
        self.revoked_at.is_none() && Utc::now() < self.expires_at
    }

    /// トークンを無効化
    pub fn revoke(&mut self) {
        self.revoked_at = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_refresh_token() {
        let user_id = UserId::new();
        let token = RefreshToken::new(user_id, "hashed_token".to_string(), 30);

        assert_eq!(token.user_id, user_id);
        assert_eq!(token.token_hash, "hashed_token");
        assert!(token.is_valid());
        assert!(token.revoked_at.is_none());
    }

    #[test]
    fn test_revoke_token() {
        let user_id = UserId::new();
        let mut token = RefreshToken::new(user_id, "hashed_token".to_string(), 30);

        assert!(token.is_valid());

        token.revoke();
        assert!(!token.is_valid());
        assert!(token.revoked_at.is_some());
    }
}
