use crate::entities::HuntingRecord;
use crate::value_objects::{HuntingRecordId, UserId};
use async_trait::async_trait;
use chrono::NaiveDate;
use shared::error::AppError;

/// 出猟記録リポジトリインターフェース
#[async_trait]
pub trait HuntingRecordRepository: Send + Sync {
    /// IDで出猟記録を検索
    async fn find_by_id(&self, id: &HuntingRecordId) -> Result<Option<HuntingRecord>, AppError>;

    /// ユーザーの出猟記録一覧を取得（削除されていないもののみ）
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<HuntingRecord>, AppError>;

    /// 日付範囲で出猟記録を検索（カレンダー表示用）
    async fn find_by_user_and_date_range(
        &self,
        user_id: &UserId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<HuntingRecord>, AppError>;

    /// 出猟記録を保存（新規作成・更新）
    async fn save(&self, record: &HuntingRecord) -> Result<(), AppError>;

    /// 出猟記録を削除（論理削除）
    async fn delete(&self, id: &HuntingRecordId) -> Result<(), AppError>;
}
