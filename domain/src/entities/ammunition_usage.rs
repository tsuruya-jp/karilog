use crate::value_objects::{AmmunitionTypeId, AmmunitionUsageId, FirearmId, UserId};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 実包使用記録エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionUsage {
    pub id: AmmunitionUsageId,
    pub user_id: UserId,
    pub ammunition_type_id: AmmunitionTypeId,
    pub firearm_id: Option<FirearmId>,
    pub hunting_record_id: Option<Uuid>,
    pub usage_date: NaiveDate,
    pub location: Option<String>,
    pub quantity_used: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl AmmunitionUsage {
    /// 新しい実包使用記録を作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
        firearm_id: Option<FirearmId>,
        hunting_record_id: Option<Uuid>,
        usage_date: NaiveDate,
        location: Option<String>,
        quantity_used: i32,
        notes: Option<String>,
    ) -> Result<Self, String> {
        if quantity_used <= 0 {
            return Err("quantity_used must be greater than 0".to_string());
        }

        let now = Utc::now();
        Ok(Self {
            id: AmmunitionUsageId::new(),
            user_id,
            ammunition_type_id,
            firearm_id,
            hunting_record_id,
            usage_date,
            location,
            quantity_used,
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

    /// 使用記録を更新
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        ammunition_type_id: AmmunitionTypeId,
        firearm_id: Option<FirearmId>,
        hunting_record_id: Option<Uuid>,
        usage_date: NaiveDate,
        location: Option<String>,
        quantity_used: i32,
        notes: Option<String>,
    ) -> Result<(), String> {
        if quantity_used <= 0 {
            return Err("quantity_used must be greater than 0".to_string());
        }

        self.ammunition_type_id = ammunition_type_id;
        self.firearm_id = firearm_id;
        self.hunting_record_id = hunting_record_id;
        self.usage_date = usage_date;
        self.location = location;
        self.quantity_used = quantity_used;
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
    fn test_new_ammunition_usage() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let firearm_id = Some(FirearmId::new());
        let usage_date = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();

        let usage = AmmunitionUsage::new(
            user_id,
            ammunition_type_id,
            firearm_id,
            None,
            usage_date,
            Some("射撃場".to_string()),
            50,
            None,
        )
        .unwrap();

        assert_eq!(usage.user_id, user_id);
        assert_eq!(usage.ammunition_type_id, ammunition_type_id);
        assert_eq!(usage.quantity_used, 50);
        assert_eq!(usage.location, Some("射撃場".to_string()));
        assert!(!usage.is_deleted());
    }

    #[test]
    fn test_new_ammunition_usage_invalid_quantity() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let usage_date = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();

        let result = AmmunitionUsage::new(
            user_id,
            ammunition_type_id,
            None,
            None,
            usage_date,
            None,
            0,
            None,
        );
        assert!(result.is_err());

        let result = AmmunitionUsage::new(
            user_id,
            ammunition_type_id,
            None,
            None,
            usage_date,
            None,
            -10,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_ammunition_usage() {
        let user_id = UserId::new();
        let ammunition_type_id = AmmunitionTypeId::new();
        let usage_date = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();

        let mut usage = AmmunitionUsage::new(
            user_id,
            ammunition_type_id,
            None,
            None,
            usage_date,
            Some("射撃場".to_string()),
            50,
            None,
        )
        .unwrap();

        usage.delete();
        assert!(usage.is_deleted());
        assert!(usage.deleted_at.is_some());
    }
}
