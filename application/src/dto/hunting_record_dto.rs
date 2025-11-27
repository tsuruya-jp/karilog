use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 出猟記録作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateHuntingRecordRequest {
    pub hunting_date: NaiveDate,
    pub location: Option<String>,
    #[serde(default = "default_is_planned")]
    pub is_planned: bool,
    pub notes: Option<String>,
}

fn default_is_planned() -> bool {
    true
}

/// 出猟記録更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateHuntingRecordRequest {
    pub hunting_date: NaiveDate,
    pub location: Option<String>,
    pub is_planned: bool,
    pub notes: Option<String>,
}

/// 出猟記録レスポンス
#[derive(Debug, Serialize)]
pub struct HuntingRecordResponse {
    pub id: String,
    pub hunting_date: NaiveDate,
    pub location: Option<String>,
    pub is_planned: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 出猟記録一覧クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct ListHuntingRecordsQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_planned: Option<bool>,
}

/// 出猟統計レスポンス
#[derive(Debug, Serialize)]
pub struct HuntingStatisticsResponse {
    pub total_hunting_days: i64,
    pub planned_count: i64,
    pub completed_count: i64,
    pub current_year_count: i64,
    pub current_month_count: i64,
}

impl From<domain::entities::HuntingRecord> for HuntingRecordResponse {
    fn from(record: domain::entities::HuntingRecord) -> Self {
        Self {
            id: record.id.to_string(),
            hunting_date: record.hunting_date,
            location: record.location,
            is_planned: record.is_planned,
            notes: record.notes,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}
