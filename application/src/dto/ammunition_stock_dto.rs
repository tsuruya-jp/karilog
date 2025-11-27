use domain::services::AmmunitionStock;
use serde::Serialize;

/// 実包在庫レスポンス
#[derive(Debug, Serialize)]
pub struct AmmunitionStockResponse {
    pub ammunition_type_id: String,
    pub ammunition_type_name: String,
    pub caliber: String,
    pub total_purchased: i32,
    pub total_used: i32,
    pub current_stock: i32,
    pub max_quantity: Option<i32>,
    pub remaining_quota: Option<i32>,
}

impl From<AmmunitionStock> for AmmunitionStockResponse {
    fn from(stock: AmmunitionStock) -> Self {
        Self {
            ammunition_type_id: stock.ammunition_type_id.to_string(),
            ammunition_type_name: stock.ammunition_type_name,
            caliber: stock.caliber,
            total_purchased: stock.total_purchased,
            total_used: stock.total_used,
            current_stock: stock.current_stock,
            max_quantity: stock.max_quantity,
            remaining_quota: stock.remaining_quota,
        }
    }
}
