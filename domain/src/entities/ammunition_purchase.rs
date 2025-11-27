use crate::value_objects::{AmmunitionPurchaseId, AmmunitionTypeId, FirearmId, UserId};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 実包購入記録エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionPurchase {
    pub id: AmmunitionPurchaseId,
    pub user_id: UserId,
    pub ammunition_type_id: AmmunitionTypeId,
    pub firearm_id: Option<FirearmId>,
    pub purchase_date: NaiveDate,
    pub supplier: String,
    pub quantity: i32,
    pub price: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl AmmunitionPurchase {
    /// 新しい実包購入記録を作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
        firearm_id: Option<FirearmId>,
        purchase_date: NaiveDate,
        supplier: String,
        quantity: i32,
        price: Option<i32>,
        notes: Option<String>,
    ) -> Result<Self, String> {
        if quantity <= 0 {
            return Err("quantity must be greater than 0".to_string());
        }

        if let Some(p) = price {
            if p < 0 {
                return Err("price must be greater than or equal to 0".to_string());
            }
        }

        let now = Utc::now();
        Ok(Self {
            id: AmmunitionPurchaseId::new(),
            user_id,
            ammunition_type_id,
            firearm_id,
            purchase_date,
            supplier,
            quantity,
            price,
            notes,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    /// 削除されているか確認
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// 購入記録を更新
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        ammunition_type_id: AmmunitionTypeId,
        firearm_id: Option<FirearmId>,
        purchase_date: NaiveDate,
        supplier: String,
        quantity: i32,
        price: Option<i32>,
        notes: Option<String>,
    ) -> Result<(), String> {
        if quantity <= 0 {
            return Err("quantity must be greater than 0".to_string());
        }

        if let Some(p) = price {
            if p < 0 {
                return Err("price must be greater than or equal to 0".to_string());
            }
        }

        self.ammunition_type_id = ammunition_type_id;
        self.firearm_id = firearm_id;
        self.purchase_date = purchase_date;
        self.supplier = supplier;
        self.quantity = quantity;
        self.price = price;
        self.notes = notes;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 論理削除
    pub fn delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_new_ammunition_purchase() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let firearm_id = Some(FirearmId::new());
        let purchase_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();

        let purchase = AmmunitionPurchase::new(
            user_id,
            ammunition_type_id,
            firearm_id,
            purchase_date,
            "銃砲店A".to_string(),
            100,
            Some(10000),
            None,
        )
        .unwrap();

        assert_eq!(purchase.user_id, user_id);
        assert_eq!(purchase.ammunition_type_id, ammunition_type_id);
        assert_eq!(purchase.quantity, 100);
        assert_eq!(purchase.supplier, "銃砲店A");
        assert!(!purchase.is_deleted());
    }

    #[test]
    fn test_new_ammunition_purchase_invalid_quantity() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let purchase_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();

        let result = AmmunitionPurchase::new(
            user_id,
            ammunition_type_id,
            None,
            purchase_date,
            "銃砲店A".to_string(),
            0,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_new_ammunition_purchase_invalid_price() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let purchase_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();

        let result = AmmunitionPurchase::new(
            user_id,
            ammunition_type_id,
            None,
            purchase_date,
            "銃砲店A".to_string(),
            100,
            Some(-1000),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_ammunition_purchase() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let purchase_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();

        let mut purchase = AmmunitionPurchase::new(
            user_id,
            ammunition_type_id,
            None,
            purchase_date,
            "銃砲店A".to_string(),
            100,
            None,
            None,
        )
        .unwrap();

        purchase.delete();
        assert!(purchase.is_deleted());
        assert!(purchase.deleted_at.is_some());
    }
}
