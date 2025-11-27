use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 実包購入記録作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateAmmunitionPurchaseRequest {
    pub ammunition_type_id: String,
    pub firearm_id: Option<String>,
    pub purchase_date: NaiveDate,
    pub supplier: String,
    pub quantity: i32,
    pub price: Option<i32>,
    pub notes: Option<String>,
}

/// 実包購入記録更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateAmmunitionPurchaseRequest {
    pub ammunition_type_id: String,
    pub firearm_id: Option<String>,
    pub purchase_date: NaiveDate,
    pub supplier: String,
    pub quantity: i32,
    pub price: Option<i32>,
    pub notes: Option<String>,
}

/// 実包購入記録レスポンス
#[derive(Debug, Serialize)]
pub struct AmmunitionPurchaseResponse {
    pub id: String,
    pub ammunition_type_id: String,
    pub firearm_id: Option<String>,
    pub purchase_date: NaiveDate,
    pub supplier: String,
    pub quantity: i32,
    pub price: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<domain::entities::AmmunitionPurchase> for AmmunitionPurchaseResponse {
    fn from(ammunition_purchase: domain::entities::AmmunitionPurchase) -> Self {
        Self {
            id: ammunition_purchase.id.to_string(),
            ammunition_type_id: ammunition_purchase.ammunition_type_id.to_string(),
            firearm_id: ammunition_purchase.firearm_id.map(|id| id.to_string()),
            purchase_date: ammunition_purchase.purchase_date,
            supplier: ammunition_purchase.supplier,
            quantity: ammunition_purchase.quantity,
            price: ammunition_purchase.price,
            notes: ammunition_purchase.notes,
            created_at: ammunition_purchase.created_at,
            updated_at: ammunition_purchase.updated_at,
        }
    }
}
