use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 実包使用記録作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateAmmunitionUsageRequest {
    pub ammunition_type_id: String,
    pub firearm_id: Option<String>,
    pub hunting_record_id: Option<String>,
    pub usage_date: NaiveDate,
    pub location: Option<String>,
    pub quantity_used: i32,
    pub notes: Option<String>,
}

/// 実包使用記録更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateAmmunitionUsageRequest {
    pub ammunition_type_id: String,
    pub firearm_id: Option<String>,
    pub hunting_record_id: Option<String>,
    pub usage_date: NaiveDate,
    pub location: Option<String>,
    pub quantity_used: i32,
    pub notes: Option<String>,
}

/// 実包使用記録レスポンス
#[derive(Debug, Serialize)]
pub struct AmmunitionUsageResponse {
    pub id: String,
    pub ammunition_type_id: String,
    pub firearm_id: Option<String>,
    pub hunting_record_id: Option<String>,
    pub usage_date: NaiveDate,
    pub location: Option<String>,
    pub quantity_used: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<domain::entities::AmmunitionUsage> for AmmunitionUsageResponse {
    fn from(ammunition_usage: domain::entities::AmmunitionUsage) -> Self {
        Self {
            id: ammunition_usage.id.to_string(),
            ammunition_type_id: ammunition_usage.ammunition_type_id.to_string(),
            firearm_id: ammunition_usage.firearm_id.map(|id| id.to_string()),
            hunting_record_id: ammunition_usage.hunting_record_id.map(|id| id.to_string()),
            usage_date: ammunition_usage.usage_date,
            location: ammunition_usage.location,
            quantity_used: ammunition_usage.quantity_used,
            notes: ammunition_usage.notes,
            created_at: ammunition_usage.created_at,
            updated_at: ammunition_usage.updated_at,
        }
    }
}
