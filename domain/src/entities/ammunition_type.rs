use crate::value_objects::{AmmunitionTypeId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 実包種別エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionType {
    pub id: AmmunitionTypeId,
    pub user_id: UserId,
    pub name: String,
    pub caliber: String,
    pub shot_size: Option<String>,
    pub is_slug: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl AmmunitionType {
    /// 新しい実包種別を作成
    pub fn new(
        user_id: UserId,
        name: String,
        caliber: String,
        shot_size: Option<String>,
        is_slug: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: AmmunitionTypeId::new(),
            user_id,
            name,
            caliber,
            shot_size,
            is_slug,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// 削除されているか確認
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// 実包種別情報を更新
    pub fn update(
        &mut self,
        name: String,
        caliber: String,
        shot_size: Option<String>,
        is_slug: bool,
    ) {
        self.name = name;
        self.caliber = caliber;
        self.shot_size = shot_size;
        self.is_slug = is_slug;
        self.updated_at = Utc::now();
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

    #[test]
    fn test_new_ammunition_type() {
        let user_id = UserId::new();
        let ammunition_type = AmmunitionType::new(
            user_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            Some("7.5号".to_string()),
            false,
        );

        assert_eq!(ammunition_type.user_id, user_id);
        assert_eq!(ammunition_type.name, "12番 7.5号");
        assert_eq!(ammunition_type.caliber, "12番");
        assert_eq!(ammunition_type.shot_size, Some("7.5号".to_string()));
        assert!(!ammunition_type.is_slug);
        assert!(!ammunition_type.is_deleted());
    }

    #[test]
    fn test_slug_ammunition_type() {
        let user_id = UserId::new();
        let ammunition_type = AmmunitionType::new(
            user_id,
            "12番 スラッグ".to_string(),
            "12番".to_string(),
            None,
            true,
        );

        assert!(ammunition_type.is_slug);
        assert_eq!(ammunition_type.shot_size, None);
    }

    #[test]
    fn test_update_ammunition_type() {
        let user_id = UserId::new();
        let mut ammunition_type = AmmunitionType::new(
            user_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            Some("7.5号".to_string()),
            false,
        );

        ammunition_type.update(
            "12番 9号".to_string(),
            "12番".to_string(),
            Some("9号".to_string()),
            false,
        );

        assert_eq!(ammunition_type.name, "12番 9号");
        assert_eq!(ammunition_type.shot_size, Some("9号".to_string()));
    }

    #[test]
    fn test_delete_ammunition_type() {
        let user_id = UserId::new();
        let mut ammunition_type = AmmunitionType::new(
            user_id,
            "12番 7.5号".to_string(),
            "12番".to_string(),
            Some("7.5号".to_string()),
            false,
        );

        ammunition_type.delete();
        assert!(ammunition_type.is_deleted());
        assert!(ammunition_type.deleted_at.is_some());
    }
}
