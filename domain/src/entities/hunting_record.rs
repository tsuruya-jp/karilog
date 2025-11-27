use crate::value_objects::{HuntingRecordId, UserId};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 出猟記録エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuntingRecord {
    pub id: HuntingRecordId,
    pub user_id: UserId,
    pub hunting_date: NaiveDate,
    pub location: Option<String>,
    pub is_planned: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl HuntingRecord {
    /// 新しい出猟記録を作成
    pub fn new(
        user_id: UserId,
        hunting_date: NaiveDate,
        location: Option<String>,
        is_planned: bool,
        notes: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: HuntingRecordId::new(),
            user_id,
            hunting_date,
            location,
            is_planned,
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

    /// 出猟記録を更新
    pub fn update(
        &mut self,
        hunting_date: NaiveDate,
        location: Option<String>,
        is_planned: bool,
        notes: Option<String>,
    ) {
        self.hunting_date = hunting_date;
        self.location = location;
        self.is_planned = is_planned;
        self.notes = notes;
        self.updated_at = Utc::now();
    }

    /// 予定から実績に変更
    pub fn mark_as_completed(&mut self) {
        if self.is_planned {
            self.is_planned = false;
            self.updated_at = Utc::now();
        }
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
    fn test_new_hunting_record() {
        let user_id = UserId::new();
        let hunting_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let record = HuntingRecord::new(
            user_id,
            hunting_date,
            Some("山田猟区".to_string()),
            true,
            Some("初猟".to_string()),
        );

        assert_eq!(record.user_id, user_id);
        assert_eq!(record.hunting_date, hunting_date);
        assert_eq!(record.location, Some("山田猟区".to_string()));
        assert!(record.is_planned);
        assert!(!record.is_deleted());
    }

    #[test]
    fn test_update_hunting_record() {
        let user_id = UserId::new();
        let hunting_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let mut record = HuntingRecord::new(
            user_id,
            hunting_date,
            Some("山田猟区".to_string()),
            true,
            None,
        );

        let new_date = NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();
        record.update(
            new_date,
            Some("鈴木猟区".to_string()),
            false,
            Some("実績記録".to_string()),
        );

        assert_eq!(record.hunting_date, new_date);
        assert_eq!(record.location, Some("鈴木猟区".to_string()));
        assert!(!record.is_planned);
    }

    #[test]
    fn test_mark_as_completed() {
        let user_id = UserId::new();
        let hunting_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let mut record = HuntingRecord::new(user_id, hunting_date, None, true, None);

        assert!(record.is_planned);
        record.mark_as_completed();
        assert!(!record.is_planned);
    }

    #[test]
    fn test_delete_hunting_record() {
        let user_id = UserId::new();
        let hunting_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let mut record = HuntingRecord::new(user_id, hunting_date, None, true, None);

        record.delete();
        assert!(record.is_deleted());
        assert!(record.deleted_at.is_some());
    }
}
