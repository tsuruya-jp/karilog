use chrono::{Duration, Utc};
use domain::value_objects::UserId;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared::error::AppError;

/// JWTトークンのクレーム
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     // Subject (ユーザーID)
    pub exp: i64,        // Expiration time
    pub iat: i64,        // Issued at
    pub token_type: String, // "access" or "refresh"
}

/// JWTサービス
#[derive(Clone)]
pub struct JwtService {
    secret: String,
    access_token_expires_in_minutes: i64,
    refresh_token_expires_in_days: i64,
}

impl JwtService {
    /// 新しいJWTサービスを作成
    pub fn new(
        secret: String,
        access_token_expires_in_minutes: i64,
        refresh_token_expires_in_days: i64,
    ) -> Self {
        Self {
            secret,
            access_token_expires_in_minutes,
            refresh_token_expires_in_days,
        }
    }

    /// アクセストークンを生成
    pub fn generate_access_token(&self, user_id: &UserId) -> Result<String, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::minutes(self.access_token_expires_in_minutes);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
    }

    /// リフレッシュトークンを生成
    pub fn generate_refresh_token(&self, user_id: &UserId) -> Result<String, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::days(self.refresh_token_expires_in_days);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Failed to generate token: {}", e)))
    }

    /// トークンを検証してクレームを取得
    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }

    /// アクセストークンを検証してユーザーIDを取得
    pub fn verify_access_token(&self, token: &str) -> Result<UserId, AppError> {
        let claims = self.verify_token(token)?;

        if claims.token_type != "access" {
            return Err(AppError::Unauthorized(
                "Invalid token type".to_string(),
            ));
        }

        UserId::from_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))
    }

    /// リフレッシュトークンを検証してユーザーIDを取得
    pub fn verify_refresh_token(&self, token: &str) -> Result<UserId, AppError> {
        let claims = self.verify_token(token)?;

        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized(
                "Invalid token type".to_string(),
            ));
        }

        UserId::from_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))
    }

    /// リフレッシュトークンの有効期限（日数）を取得
    pub fn get_refresh_token_expires_in_days(&self) -> i64 {
        self.refresh_token_expires_in_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> JwtService {
        JwtService::new("test_secret".to_string(), 15, 30)
    }

    #[test]
    fn test_generate_and_verify_access_token() {
        let service = create_test_service();
        let user_id = UserId::new();

        let token = service.generate_access_token(&user_id).unwrap();
        let verified_user_id = service.verify_access_token(&token).unwrap();

        assert_eq!(user_id, verified_user_id);
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let service = create_test_service();
        let user_id = UserId::new();

        let token = service.generate_refresh_token(&user_id).unwrap();
        let verified_user_id = service.verify_refresh_token(&token).unwrap();

        assert_eq!(user_id, verified_user_id);
    }

    #[test]
    fn test_access_token_cannot_be_used_as_refresh_token() {
        let service = create_test_service();
        let user_id = UserId::new();

        let access_token = service.generate_access_token(&user_id).unwrap();
        let result = service.verify_refresh_token(&access_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token_cannot_be_used_as_access_token() {
        let service = create_test_service();
        let user_id = UserId::new();

        let refresh_token = service.generate_refresh_token(&user_id).unwrap();
        let result = service.verify_access_token(&refresh_token);

        assert!(result.is_err());
    }
}
