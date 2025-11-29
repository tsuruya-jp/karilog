use crate::entities::AmmunitionPurchase;
use crate::value_objects::{AmmunitionPurchaseId, AmmunitionTypeId, UserId};
use async_trait::async_trait;
use chrono::NaiveDate;
use shared::error::AppError;

/// 実包購入記録リポジトリインターフェース
#[async_trait]
pub trait AmmunitionPurchaseRepository: Send + Sync {
    /// IDで実包購入記録を検索
    async fn find_by_id(
        &self,
        id: &AmmunitionPurchaseId,
    ) -> Result<Option<AmmunitionPurchase>, AppError>;

    /// ユーザーの実包購入記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionPurchase>, AppError>;

    /// ユーザーの日付範囲内の実包購入記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id_and_date_range(
        &self,
        user_id: &UserId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AmmunitionPurchase>, AppError>;

    /// ユーザーの実包種別別購入記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id_and_ammunition_type_id(
        &self,
        user_id: &UserId,
        ammunition_type_id: &AmmunitionTypeId,
    ) -> Result<Vec<AmmunitionPurchase>, AppError>;

    /// 実包購入記録を保存（新規作成・更新）
    async fn save(&self, ammunition_purchase: &AmmunitionPurchase) -> Result<(), AppError>;

    /// 実包購入記録を削除（論理削除）
    async fn delete(&self, id: &AmmunitionPurchaseId) -> Result<(), AppError>;
}
