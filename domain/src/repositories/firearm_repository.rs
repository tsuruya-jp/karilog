use crate::entities::Firearm;
use crate::value_objects::{FirearmId, UserId};
use async_trait::async_trait;
use shared::error::AppError;

/// 銃砲リポジトリインターフェース
#[async_trait]
pub trait FirearmRepository: Send + Sync {
    /// IDで銃砲を検索
    async fn find_by_id(&self, id: &FirearmId) -> Result<Option<Firearm>, AppError>;

    /// ユーザーの銃砲一覧を取得（削除されていないもののみ）
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Firearm>, AppError>;

    /// 銃砲を保存（新規作成・更新）
    async fn save(&self, firearm: &Firearm) -> Result<(), AppError>;

    /// 銃砲を削除（論理削除）
    async fn delete(&self, id: &FirearmId) -> Result<(), AppError>;
}
