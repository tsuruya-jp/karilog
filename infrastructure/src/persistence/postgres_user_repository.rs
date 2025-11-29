use async_trait::async_trait;
use domain::entities::{RefreshToken, User};
use domain::repositories::UserRepository;
use domain::value_objects::{Email, PasswordHash, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

/// PostgreSQL実装のUserRepository
#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, name, email_verified,
                   email_verification_token, email_verification_expires_at,
                   password_reset_token, password_reset_expires_at,
                   failed_login_attempts, locked_until,
                   created_at, updated_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let user = User {
                    id: UserId::from_uuid(row.get("id")),
                    email: Email::new(row.get("email")).map_err(|e| {
                        AppError::ValidationError(format!("Invalid email: {:?}", e))
                    })?,
                    password_hash: PasswordHash::new(row.get("password_hash")),
                    name: row.get("name"),
                    email_verified: row.get("email_verified"),
                    email_verification_token: row.get("email_verification_token"),
                    email_verification_expires_at: row.get("email_verification_expires_at"),
                    password_reset_token: row.get("password_reset_token"),
                    password_reset_expires_at: row.get("password_reset_expires_at"),
                    failed_login_attempts: row.get("failed_login_attempts"),
                    locked_until: row.get("locked_until"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, name, email_verified,
                   email_verification_token, email_verification_expires_at,
                   password_reset_token, password_reset_expires_at,
                   failed_login_attempts, locked_until,
                   created_at, updated_at, deleted_at
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let user = User {
                    id: UserId::from_uuid(row.get("id")),
                    email: Email::new(row.get("email")).map_err(|e| {
                        AppError::ValidationError(format!("Invalid email: {:?}", e))
                    })?,
                    password_hash: PasswordHash::new(row.get("password_hash")),
                    name: row.get("name"),
                    email_verified: row.get("email_verified"),
                    email_verification_token: row.get("email_verification_token"),
                    email_verification_expires_at: row.get("email_verification_expires_at"),
                    password_reset_token: row.get("password_reset_token"),
                    password_reset_expires_at: row.get("password_reset_expires_at"),
                    failed_login_attempts: row.get("failed_login_attempts"),
                    locked_until: row.get("locked_until"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email_verification_token(
        &self,
        token: &str,
    ) -> Result<Option<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, name, email_verified,
                   email_verification_token, email_verification_expires_at,
                   password_reset_token, password_reset_expires_at,
                   failed_login_attempts, locked_until,
                   created_at, updated_at, deleted_at
            FROM users
            WHERE email_verification_token = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let user = User {
                    id: UserId::from_uuid(row.get("id")),
                    email: Email::new(row.get("email")).map_err(|e| {
                        AppError::ValidationError(format!("Invalid email: {:?}", e))
                    })?,
                    password_hash: PasswordHash::new(row.get("password_hash")),
                    name: row.get("name"),
                    email_verified: row.get("email_verified"),
                    email_verification_token: row.get("email_verification_token"),
                    email_verification_expires_at: row.get("email_verification_expires_at"),
                    password_reset_token: row.get("password_reset_token"),
                    password_reset_expires_at: row.get("password_reset_expires_at"),
                    failed_login_attempts: row.get("failed_login_attempts"),
                    locked_until: row.get("locked_until"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_password_reset_token(&self, token: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, name, email_verified,
                   email_verification_token, email_verification_expires_at,
                   password_reset_token, password_reset_expires_at,
                   failed_login_attempts, locked_until,
                   created_at, updated_at, deleted_at
            FROM users
            WHERE password_reset_token = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let user = User {
                    id: UserId::from_uuid(row.get("id")),
                    email: Email::new(row.get("email")).map_err(|e| {
                        AppError::ValidationError(format!("Invalid email: {:?}", e))
                    })?,
                    password_hash: PasswordHash::new(row.get("password_hash")),
                    name: row.get("name"),
                    email_verified: row.get("email_verified"),
                    email_verification_token: row.get("email_verification_token"),
                    email_verification_expires_at: row.get("email_verification_expires_at"),
                    password_reset_token: row.get("password_reset_token"),
                    password_reset_expires_at: row.get("password_reset_expires_at"),
                    failed_login_attempts: row.get("failed_login_attempts"),
                    locked_until: row.get("locked_until"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id, email, password_hash, name, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                failed_login_attempts, locked_until,
                created_at, updated_at, deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                name = EXCLUDED.name,
                email_verified = EXCLUDED.email_verified,
                email_verification_token = EXCLUDED.email_verification_token,
                email_verification_expires_at = EXCLUDED.email_verification_expires_at,
                password_reset_token = EXCLUDED.password_reset_token,
                password_reset_expires_at = EXCLUDED.password_reset_expires_at,
                failed_login_attempts = EXCLUDED.failed_login_attempts,
                locked_until = EXCLUDED.locked_until,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
        )
        .bind(user.id.as_uuid())
        .bind(user.email.as_str())
        .bind(user.password_hash.as_str())
        .bind(&user.name)
        .bind(user.email_verified)
        .bind(&user.email_verification_token)
        .bind(user.email_verification_expires_at)
        .bind(&user.password_reset_token)
        .bind(user.password_reset_expires_at)
        .bind(user.failed_login_attempts)
        .bind(user.locked_until)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_refresh_token_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at, revoked_at
            FROM refresh_tokens
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let token = RefreshToken {
                    id: row.get("id"),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    token_hash: row.get("token_hash"),
                    expires_at: row.get("expires_at"),
                    created_at: row.get("created_at"),
                    revoked_at: row.get("revoked_at"),
                };
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    async fn find_refresh_token_by_id(&self, id: &Uuid) -> Result<Option<RefreshToken>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at, revoked_at
            FROM refresh_tokens
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let token = RefreshToken {
                    id: row.get("id"),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    token_hash: row.get("token_hash"),
                    expires_at: row.get("expires_at"),
                    created_at: row.get("created_at"),
                    revoked_at: row.get("revoked_at"),
                };
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    async fn find_valid_refresh_tokens_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<RefreshToken>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at, revoked_at
            FROM refresh_tokens
            WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > NOW()
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let tokens = rows
            .into_iter()
            .map(|row| RefreshToken {
                id: row.get("id"),
                user_id: UserId::from_uuid(row.get("user_id")),
                token_hash: row.get("token_hash"),
                expires_at: row.get("expires_at"),
                created_at: row.get("created_at"),
                revoked_at: row.get("revoked_at"),
            })
            .collect();

        Ok(tokens)
    }

    async fn save_refresh_token(&self, token: &RefreshToken) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (
                id, user_id, token_hash, expires_at, created_at, revoked_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                token_hash = EXCLUDED.token_hash,
                expires_at = EXCLUDED.expires_at,
                revoked_at = EXCLUDED.revoked_at
            "#,
        )
        .bind(token.id)
        .bind(token.user_id.as_uuid())
        .bind(&token.token_hash)
        .bind(token.expires_at)
        .bind(token.created_at)
        .bind(token.revoked_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_refresh_token(&self, id: &Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM refresh_tokens
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn revoke_all_refresh_tokens_for_user(&self, user_id: &UserId) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(user_id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
