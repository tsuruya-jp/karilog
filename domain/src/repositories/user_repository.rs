use crate::entities::{RefreshToken, User};
use crate::value_objects::{Email, UserId};
use async_trait::async_trait;
use shared::error::AppError;
use uuid::Uuid;

/// ユーザーリポジトリインターフェース
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// IDでユーザーを検索
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AppError>;

    /// メールアドレスでユーザーを検索（削除されていないユーザーのみ）
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError>;

    /// メール確認トークンでユーザーを検索
    async fn find_by_email_verification_token(&self, token: &str)
        -> Result<Option<User>, AppError>;

    /// パスワードリセットトークンでユーザーを検索
    async fn find_by_password_reset_token(&self, token: &str) -> Result<Option<User>, AppError>;

    /// ユーザーを保存（新規作成・更新）
    async fn save(&self, user: &User) -> Result<(), AppError>;

    /// ユーザーを削除（論理削除）
    async fn delete(&self, id: &UserId) -> Result<(), AppError>;

    /// リフレッシュトークンをトークンハッシュで検索
    async fn find_refresh_token_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, AppError>;

    /// リフレッシュトークンをIDで検索
    async fn find_refresh_token_by_id(&self, id: &Uuid) -> Result<Option<RefreshToken>, AppError>;

    /// ユーザーの有効なリフレッシュトークンを全て取得
    async fn find_valid_refresh_tokens_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<RefreshToken>, AppError>;

    /// リフレッシュトークンを保存（新規作成・更新）
    async fn save_refresh_token(&self, token: &RefreshToken) -> Result<(), AppError>;

    /// リフレッシュトークンを削除
    async fn delete_refresh_token(&self, id: &Uuid) -> Result<(), AppError>;

    /// ユーザーの全てのリフレッシュトークンを無効化
    async fn revoke_all_refresh_tokens_for_user(&self, user_id: &UserId) -> Result<(), AppError>;
}
