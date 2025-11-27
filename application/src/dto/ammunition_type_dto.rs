use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 実包種別作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateAmmunitionTypeRequest {
    pub name: String,
    pub caliber: String,
    pub shot_size: Option<String>,
    pub is_slug: bool,
}

/// 実包種別更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateAmmunitionTypeRequest {
    pub name: String,
    pub caliber: String,
    pub shot_size: Option<String>,
    pub is_slug: bool,
}

/// 実包種別レスポンス
#[derive(Debug, Serialize)]
pub struct AmmunitionTypeResponse {
    pub id: String,
    pub name: String,
    pub caliber: String,
    pub shot_size: Option<String>,
    pub is_slug: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<domain::entities::AmmunitionType> for AmmunitionTypeResponse {
    fn from(ammunition_type: domain::entities::AmmunitionType) -> Self {
        Self {
            id: ammunition_type.id.to_string(),
            name: ammunition_type.name,
            caliber: ammunition_type.caliber,
            shot_size: ammunition_type.shot_size,
            is_slug: ammunition_type.is_slug,
            created_at: ammunition_type.created_at,
            updated_at: ammunition_type.updated_at,
        }
    }
}
