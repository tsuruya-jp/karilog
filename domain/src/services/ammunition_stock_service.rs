use crate::repositories::{
    AmmunitionLimitRepository, AmmunitionPurchaseRepository, AmmunitionTypeRepository,
    AmmunitionUsageRepository,
};
use crate::value_objects::{AmmunitionTypeId, UserId};
use serde::{Deserialize, Serialize};
use shared::error::AppError;
use std::sync::Arc;

/// 実包在庫情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionStock {
    pub user_id: UserId,
    pub ammunition_type_id: AmmunitionTypeId,
    pub ammunition_type_name: String,
    pub caliber: String,
    pub total_purchased: i32,
    pub total_used: i32,
    pub current_stock: i32,
    pub max_quantity: Option<i32>,
    pub remaining_quota: Option<i32>,
}

impl AmmunitionStock {
    /// 新しい実包在庫情報を作成
    pub fn new(
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
        ammunition_type_name: String,
        caliber: String,
        total_purchased: i32,
        total_used: i32,
        max_quantity: Option<i32>,
    ) -> Self {
        let current_stock = total_purchased - total_used;
        let remaining_quota = max_quantity.map(|max| max - current_stock);

        Self {
            user_id,
            ammunition_type_id,
            ammunition_type_name,
            caliber,
            total_purchased,
            total_used,
            current_stock,
            max_quantity,
            remaining_quota,
        }
    }

    /// 在庫数を計算
    pub fn calculate_current_stock(&self) -> i32 {
        self.total_purchased - self.total_used
    }

    /// 残りの譲受可能数を計算
    pub fn calculate_remaining_quota(&self) -> Option<i32> {
        self.max_quantity.map(|max| max - self.current_stock)
    }

    /// 指定数量の購入が許可上限を超えるか確認
    pub fn would_exceed_limit(&self, purchase_quantity: i32) -> bool {
        if let Some(remaining) = self.remaining_quota {
            purchase_quantity > remaining
        } else {
            false
        }
    }

    /// 在庫が不足しているか確認
    pub fn is_insufficient_stock(&self, usage_quantity: i32) -> bool {
        usage_quantity > self.current_stock
    }

    /// 在庫が負になっているか確認
    pub fn has_negative_stock(&self) -> bool {
        self.current_stock < 0
    }
}

/// 実包在庫ドメインサービス
pub struct AmmunitionStockService {
    ammunition_type_repository: Arc<dyn AmmunitionTypeRepository>,
    ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
    ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
    ammunition_limit_repository: Arc<dyn AmmunitionLimitRepository>,
}

impl AmmunitionStockService {
    pub fn new(
        ammunition_type_repository: Arc<dyn AmmunitionTypeRepository>,
        ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
        ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
        ammunition_limit_repository: Arc<dyn AmmunitionLimitRepository>,
    ) -> Self {
        Self {
            ammunition_type_repository,
            ammunition_purchase_repository,
            ammunition_usage_repository,
            ammunition_limit_repository,
        }
    }

    /// すべての実包種別の在庫を計算
    pub async fn calculate_all_stocks(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AmmunitionStock>, AppError> {
        // ユーザーのすべての実包種別を取得
        let ammunition_types = self
            .ammunition_type_repository
            .find_by_user_id(&user_id)
            .await?;

        let mut stocks = Vec::new();

        for ammunition_type in ammunition_types {
            let stock = self
                .calculate_stock(user_id, ammunition_type.id.clone())
                .await?;
            stocks.push(stock);
        }

        Ok(stocks)
    }

    /// 特定の実包種別の在庫を計算
    pub async fn calculate_stock(
        &self,
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
    ) -> Result<AmmunitionStock, AppError> {
        // 実包種別を取得
        let ammunition_type = self
            .ammunition_type_repository
            .find_by_id(&ammunition_type_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Ammunition type not found".to_string()))?;

        // 購入合計を計算
        let purchases = self
            .ammunition_purchase_repository
            .find_by_user_id_and_ammunition_type_id(&user_id, &ammunition_type_id)
            .await?;
        let total_purchased: i32 = purchases.iter().map(|p| p.quantity).sum();

        // 使用合計を計算
        let usages = self
            .ammunition_usage_repository
            .find_by_user_id_and_ammunition_type_id(&user_id, &ammunition_type_id)
            .await?;
        let total_used: i32 = usages.iter().map(|u| u.quantity_used).sum();

        // 許可上限を取得
        let limit = self
            .ammunition_limit_repository
            .find_by_user_id_and_caliber(&user_id, &ammunition_type.caliber)
            .await?;
        let max_quantity = limit.map(|l| l.max_quantity);

        // 在庫情報を作成
        let stock = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            ammunition_type.name,
            ammunition_type.caliber,
            total_purchased,
            total_used,
            max_quantity,
        );

        Ok(stock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ammunition_stock() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let stock = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            500,
            200,
            Some(800),
        );

        assert_eq!(stock.current_stock, 300);
        assert_eq!(stock.remaining_quota, Some(500));
    }

    #[test]
    fn test_ammunition_stock_without_limit() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let stock = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            500,
            200,
            None,
        );

        assert_eq!(stock.current_stock, 300);
        assert_eq!(stock.remaining_quota, None);
    }

    #[test]
    fn test_would_exceed_limit() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let stock = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            500,
            200,
            Some(800),
        );

        assert!(!stock.would_exceed_limit(400));
        assert!(!stock.would_exceed_limit(500));
        assert!(stock.would_exceed_limit(501));
    }

    #[test]
    fn test_is_insufficient_stock() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let stock = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            500,
            200,
            Some(800),
        );

        assert!(!stock.is_insufficient_stock(200));
        assert!(!stock.is_insufficient_stock(300));
        assert!(stock.is_insufficient_stock(301));
    }

    #[test]
    fn test_has_negative_stock() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();

        let stock_positive = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            500,
            200,
            Some(800),
        );
        assert!(!stock_positive.has_negative_stock());

        let stock_negative = AmmunitionStock::new(
            user_id,
            ammunition_type_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            100,
            200,
            Some(800),
        );
        assert!(stock_negative.has_negative_stock());
    }
}
