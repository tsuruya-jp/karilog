use crate::entities::AmmunitionLimit;
use crate::value_objects::{AmmunitionLimitId, UserId};
use async_trait::async_trait;
use shared::error::AppError;

/// 実包所持許可上限リポジトリインターフェース
#[async_trait]
pub trait AmmunitionLimitRepository: Send + Sync {
    /// IDで実包所持許可上限を検索
    async fn find_by_id(&self, id: &AmmunitionLimitId)
        -> Result<Option<AmmunitionLimit>, AppError>;

    /// ユーザーの実包所持許可上限一覧を取得
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionLimit>, AppError>;

    /// ユーザーの口径別実包所持許可上限を取得
    async fn find_by_user_id_and_caliber(
        &self,
        user_id: &UserId,
        caliber: &str,
    ) -> Result<Option<AmmunitionLimit>, AppError>;

    /// 実包所持許可上限を保存（新規作成・更新）
    async fn save(&self, ammunition_limit: &AmmunitionLimit) -> Result<(), AppError>;

    /// 実包所持許可上限を削除
    async fn delete(&self, id: &AmmunitionLimitId) -> Result<(), AppError>;
}
