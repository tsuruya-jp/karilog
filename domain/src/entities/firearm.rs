use crate::value_objects::{FirearmId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 銃種
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FirearmType {
    Shotgun,
    Rifle,
}

impl FirearmType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FirearmType::Shotgun => "shotgun",
            FirearmType::Rifle => "rifle",
        }
    }
}

impl std::fmt::Display for FirearmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 銃砲エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Firearm {
    pub id: FirearmId,
    pub user_id: UserId,
    pub firearm_type: FirearmType,
    pub name: String,
    pub gun_number: String,
    pub permit_number: String,
    pub caliber: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Firearm {
    /// 新しい銃砲を作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: UserId,
        firearm_type: FirearmType,
        name: String,
        gun_number: String,
        permit_number: String,
        caliber: Option<String>,
        notes: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: FirearmId::new(),
            user_id,
            firearm_type,
            name,
            gun_number,
            permit_number,
            caliber,
            notes,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// 削除されているか確認
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// 銃砲情報を更新
    pub fn update(
        &mut self,
        name: String,
        gun_number: String,
        permit_number: String,
        caliber: Option<String>,
        notes: Option<String>,
    ) {
        self.name = name;
        self.gun_number = gun_number;
        self.permit_number = permit_number;
        self.caliber = caliber;
        self.notes = notes;
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
    fn test_new_firearm() {
        let user_id = UserId::new();
        let firearm = Firearm::new(
            user_id,
            FirearmType::Shotgun,
            "Browning".to_string(),
            "12345".to_string(),
            "P-12345".to_string(),
            Some("12番".to_string()),
            None,
        );

        assert_eq!(firearm.user_id, user_id);
        assert_eq!(firearm.firearm_type, FirearmType::Shotgun);
        assert_eq!(firearm.name, "Browning");
        assert!(!firearm.is_deleted());
    }

    #[test]
    fn test_update_firearm() {
        let user_id = UserId::new();
        let mut firearm = Firearm::new(
            user_id,
            FirearmType::Shotgun,
            "Browning".to_string(),
            "12345".to_string(),
            "P-12345".to_string(),
            Some("12番".to_string()),
            None,
        );

        firearm.update(
            "Remington".to_string(),
            "67890".to_string(),
            "P-67890".to_string(),
            Some("20番".to_string()),
            Some("備考".to_string()),
        );

        assert_eq!(firearm.name, "Remington");
        assert_eq!(firearm.gun_number, "67890");
        assert_eq!(firearm.caliber, Some("20番".to_string()));
    }

    #[test]
    fn test_delete_firearm() {
        let user_id = UserId::new();
        let mut firearm = Firearm::new(
            user_id,
            FirearmType::Rifle,
            "Winchester".to_string(),
            "12345".to_string(),
            "P-12345".to_string(),
            Some(".308".to_string()),
            None,
        );

        firearm.delete();
        assert!(firearm.is_deleted());
        assert!(firearm.deleted_at.is_some());
    }
}
