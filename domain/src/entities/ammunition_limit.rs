use crate::value_objects::{AmmunitionLimitId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 実包所持許可上限エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionLimit {
    pub id: AmmunitionLimitId,
    pub user_id: UserId,
    pub caliber: String,
    pub max_quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AmmunitionLimit {
    /// 新しい実包所持許可上限を作成
    pub fn new(user_id: UserId, caliber: String, max_quantity: i32) -> Result<Self, String> {
        if max_quantity <= 0 {
            return Err("max_quantity must be greater than 0".to_string());
        }

        let now = Utc::now();
        Ok(Self {
            id: AmmunitionLimitId::new(),
            user_id,
            caliber,
            max_quantity,
            created_at: now,
            updated_at: now,
        })
    }

    /// 上限数を更新
    pub fn update_max_quantity(&mut self, max_quantity: i32) -> Result<(), String> {
        if max_quantity <= 0 {
            return Err("max_quantity must be greater than 0".to_string());
        }

        self.max_quantity = max_quantity;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 残りの譲受可能数を計算
    pub fn remaining_quota(&self, current_stock: i32) -> i32 {
        self.max_quantity - current_stock
    }

    /// 指定数量の購入が許可上限を超えるか確認
    pub fn would_exceed_limit(&self, current_stock: i32, purchase_quantity: i32) -> bool {
        current_stock + purchase_quantity > self.max_quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ammunition_limit() {
        let user_id = UserId::new();
        let limit = AmmunitionLimit::new(user_id, "12番".to_string(), 800).unwrap();

        assert_eq!(limit.user_id, user_id);
        assert_eq!(limit.caliber, "12番");
        assert_eq!(limit.max_quantity, 800);
    }

    #[test]
    fn test_new_ammunition_limit_invalid() {
        let user_id = UserId::new();
        let result = AmmunitionLimit::new(user_id, "12番".to_string(), 0);
        assert!(result.is_err());

        let result = AmmunitionLimit::new(user_id, "12番".to_string(), -100);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_max_quantity() {
        let user_id = UserId::new();
        let mut limit = AmmunitionLimit::new(user_id, "12番".to_string(), 800).unwrap();

        limit.update_max_quantity(1000).unwrap();
        assert_eq!(limit.max_quantity, 1000);

        let result = limit.update_max_quantity(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_remaining_quota() {
        let user_id = UserId::new();
        let limit = AmmunitionLimit::new(user_id, "12番".to_string(), 800).unwrap();

        assert_eq!(limit.remaining_quota(200), 600);
        assert_eq!(limit.remaining_quota(800), 0);
        assert_eq!(limit.remaining_quota(900), -100);
    }

    #[test]
    fn test_would_exceed_limit() {
        let user_id = UserId::new();
        let limit = AmmunitionLimit::new(user_id, "12番".to_string(), 800).unwrap();

        assert!(!limit.would_exceed_limit(200, 500));
        assert!(!limit.would_exceed_limit(200, 600));
        assert!(limit.would_exceed_limit(200, 601));
        assert!(limit.would_exceed_limit(800, 1));
    }
}
