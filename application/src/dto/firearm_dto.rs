use chrono::{DateTime, Utc};
use domain::entities::FirearmType;
use serde::{Deserialize, Serialize};

/// 銃砲作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateFirearmRequest {
    pub firearm_type: String, // "shotgun" or "rifle"
    pub name: String,
    pub gun_number: String,
    pub permit_number: String,
    pub caliber: Option<String>,
    pub notes: Option<String>,
}

/// 銃砲更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateFirearmRequest {
    pub name: String,
    pub gun_number: String,
    pub permit_number: String,
    pub caliber: Option<String>,
    pub notes: Option<String>,
}

/// 銃砲レスポンス
#[derive(Debug, Serialize)]
pub struct FirearmResponse {
    pub id: String,
    pub firearm_type: String,
    pub name: String,
    pub gun_number: String,
    pub permit_number: String,
    pub caliber: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CreateFirearmRequest {
    pub fn parse_firearm_type(&self) -> Result<FirearmType, String> {
        match self.firearm_type.to_lowercase().as_str() {
            "shotgun" => Ok(FirearmType::Shotgun),
            "rifle" => Ok(FirearmType::Rifle),
            _ => Err(format!("Invalid firearm type: {}", self.firearm_type)),
        }
    }
}

impl From<domain::entities::Firearm> for FirearmResponse {
    fn from(firearm: domain::entities::Firearm) -> Self {
        Self {
            id: firearm.id.to_string(),
            firearm_type: firearm.firearm_type.as_str().to_string(),
            name: firearm.name,
            gun_number: firearm.gun_number,
            permit_number: firearm.permit_number,
            caliber: firearm.caliber,
            notes: firearm.notes,
            created_at: firearm.created_at,
            updated_at: firearm.updated_at,
        }
    }
}
