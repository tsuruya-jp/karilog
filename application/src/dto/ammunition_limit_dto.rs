use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 実包所持許可上限作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateAmmunitionLimitRequest {
    pub caliber: String,
    pub max_quantity: i32,
}

/// 実包所持許可上限更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateAmmunitionLimitRequest {
    pub max_quantity: i32,
}

/// 実包所持許可上限レスポンス
#[derive(Debug, Serialize)]
pub struct AmmunitionLimitResponse {
    pub id: String,
    pub caliber: String,
    pub max_quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<domain::entities::AmmunitionLimit> for AmmunitionLimitResponse {
    fn from(ammunition_limit: domain::entities::AmmunitionLimit) -> Self {
        Self {
            id: ammunition_limit.id.to_string(),
            caliber: ammunition_limit.caliber,
            max_quantity: ammunition_limit.max_quantity,
            created_at: ammunition_limit.created_at,
            updated_at: ammunition_limit.updated_at,
        }
    }
}
