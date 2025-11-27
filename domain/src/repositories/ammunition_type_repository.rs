use crate::entities::AmmunitionType;
use crate::value_objects::{AmmunitionTypeId, UserId};
use async_trait::async_trait;
use shared::error::AppError;

/// 実包種別リポジトリインターフェース
#[async_trait]
pub trait AmmunitionTypeRepository: Send + Sync {
    /// IDで実包種別を検索
    async fn find_by_id(
        &self,
        id: &AmmunitionTypeId,
    ) -> Result<Option<AmmunitionType>, AppError>;

    /// ユーザーの実包種別一覧を取得（削除されていないもののみ）
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionType>, AppError>;

    /// ユーザーの口径別実包種別一覧を取得（削除されていないもののみ）
    async fn find_by_user_id_and_caliber(
        &self,
        user_id: &UserId,
        caliber: &str,
    ) -> Result<Vec<AmmunitionType>, AppError>;

    /// 実包種別を保存（新規作成・更新）
    async fn save(&self, ammunition_type: &AmmunitionType) -> Result<(), AppError>;

    /// 実包種別を削除（論理削除）
    async fn delete(&self, id: &AmmunitionTypeId) -> Result<(), AppError>;
}
