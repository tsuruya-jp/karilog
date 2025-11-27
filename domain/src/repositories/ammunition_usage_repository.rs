use crate::entities::AmmunitionUsage;
use crate::value_objects::{AmmunitionTypeId, AmmunitionUsageId, UserId};
use async_trait::async_trait;
use chrono::NaiveDate;
use shared::error::AppError;

/// 実包使用記録リポジトリインターフェース
#[async_trait]
pub trait AmmunitionUsageRepository: Send + Sync {
    /// IDで実包使用記録を検索
    async fn find_by_id(
        &self,
        id: &AmmunitionUsageId,
    ) -> Result<Option<AmmunitionUsage>, AppError>;

    /// ユーザーの実包使用記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionUsage>, AppError>;

    /// ユーザーの日付範囲内の実包使用記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id_and_date_range(
        &self,
        user_id: &UserId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AmmunitionUsage>, AppError>;

    /// ユーザーの実包種別別使用記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id_and_ammunition_type_id(
        &self,
        user_id: &UserId,
        ammunition_type_id: &AmmunitionTypeId,
    ) -> Result<Vec<AmmunitionUsage>, AppError>;

    /// 実包使用記録を保存（新規作成・更新）
    async fn save(&self, ammunition_usage: &AmmunitionUsage) -> Result<(), AppError>;

    /// 実包使用記録を削除（論理削除）
    async fn delete(&self, id: &AmmunitionUsageId) -> Result<(), AppError>;
}
