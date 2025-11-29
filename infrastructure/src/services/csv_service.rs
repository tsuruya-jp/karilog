/// CSV生成サービスの実装
use anyhow::Result;
use async_trait::async_trait;
use domain::entities::{AmmunitionPurchase, AmmunitionUsage, HuntingRecord};
use domain::services::CsvReportService;

pub struct CsvService;

impl CsvService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CsvService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CsvReportService for CsvService {
    async fn generate_ammunition_purchases_csv(
        &self,
        purchases: Vec<AmmunitionPurchase>,
    ) -> Result<String> {
        let mut csv = String::new();

        // ヘッダー行
        csv.push_str("購入日,実包種別ID,銃砲ID,販売店,数量,価格,備考\n");

        // データ行
        for purchase in purchases {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                purchase.purchase_date.format("%Y-%m-%d"),
                purchase.ammunition_type_id,
                purchase
                    .firearm_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(String::new),
                escape_csv_field(&purchase.supplier),
                purchase.quantity,
                purchase
                    .price
                    .map(|p| p.to_string())
                    .unwrap_or_else(String::new),
                escape_csv_field(purchase.notes.as_deref().unwrap_or("")),
            ));
        }

        Ok(csv)
    }

    async fn generate_ammunition_usages_csv(&self, usages: Vec<AmmunitionUsage>) -> Result<String> {
        let mut csv = String::new();

        // ヘッダー行
        csv.push_str("使用日,実包種別ID,銃砲ID,出猟記録ID,場所,使用数,備考\n");

        // データ行
        for usage in usages {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                usage.usage_date.format("%Y-%m-%d"),
                usage.ammunition_type_id,
                usage
                    .firearm_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(String::new),
                usage
                    .hunting_record_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(String::new),
                escape_csv_field(usage.location.as_deref().unwrap_or("")),
                usage.quantity_used,
                escape_csv_field(usage.notes.as_deref().unwrap_or("")),
            ));
        }

        Ok(csv)
    }

    async fn generate_hunting_records_csv(&self, records: Vec<HuntingRecord>) -> Result<String> {
        let mut csv = String::new();

        // ヘッダー行
        csv.push_str("出猟日,場所,種別,備考\n");

        // データ行
        for record in records {
            csv.push_str(&format!(
                "{},{},{},{}\n",
                record.hunting_date.format("%Y-%m-%d"),
                escape_csv_field(record.location.as_deref().unwrap_or("")),
                if record.is_planned {
                    "予定"
                } else {
                    "実績"
                },
                escape_csv_field(record.notes.as_deref().unwrap_or("")),
            ));
        }

        Ok(csv)
    }
}

/// CSVフィールドのエスケープ処理
fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    use domain::value_objects::{
        AmmunitionPurchaseId, AmmunitionTypeId, AmmunitionUsageId, FirearmId, HuntingRecordId,
        UserId,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_generate_ammunition_purchases_csv() {
        let service = CsvService::new();
        let user_id = Uuid::new_v4();

        let purchases = vec![
            AmmunitionPurchase {
                id: AmmunitionPurchaseId(Uuid::new_v4()),
                user_id: UserId(user_id),
                ammunition_type_id: AmmunitionTypeId(Uuid::new_v4()),
                firearm_id: Some(FirearmId(Uuid::new_v4())),
                purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                supplier: "テスト販売店".to_string(),
                quantity: 50,
                price: Some(5000),
                notes: Some("テスト購入".to_string()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
            AmmunitionPurchase {
                id: AmmunitionPurchaseId(Uuid::new_v4()),
                user_id: UserId(user_id),
                ammunition_type_id: AmmunitionTypeId(Uuid::new_v4()),
                firearm_id: None,
                purchase_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
                supplier: "別の販売店".to_string(),
                quantity: 100,
                price: None,
                notes: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
        ];

        let result = service.generate_ammunition_purchases_csv(purchases).await;

        assert!(result.is_ok());
        let csv = result.unwrap();
        assert!(csv.contains("購入日,実包種別ID,銃砲ID,販売店,数量,価格,備考"));
        assert!(csv.contains("テスト販売店"));
        assert!(csv.contains("別の販売店"));
    }

    #[tokio::test]
    async fn test_generate_ammunition_usages_csv() {
        let service = CsvService::new();
        let user_id = Uuid::new_v4();

        let usages = vec![AmmunitionUsage {
            id: AmmunitionUsageId(Uuid::new_v4()),
            user_id: UserId(user_id),
            ammunition_type_id: AmmunitionTypeId(Uuid::new_v4()),
            firearm_id: Some(FirearmId(Uuid::new_v4())),
            hunting_record_id: None,
            usage_date: NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            location: Some("テスト射撃場".to_string()),
            quantity_used: 10,
            notes: Some("テスト使用".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }];

        let result = service.generate_ammunition_usages_csv(usages).await;

        assert!(result.is_ok());
        let csv = result.unwrap();
        assert!(csv.contains("使用日,実包種別ID,銃砲ID,出猟記録ID,場所,使用数,備考"));
        assert!(csv.contains("テスト射撃場"));
    }

    #[tokio::test]
    async fn test_generate_hunting_records_csv() {
        let service = CsvService::new();
        let user_id = Uuid::new_v4();

        let records = vec![
            HuntingRecord {
                id: HuntingRecordId(Uuid::new_v4()),
                user_id: UserId(user_id),
                hunting_date: NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
                location: Some("テスト猟場".to_string()),
                is_planned: false,
                notes: Some("テスト出猟".to_string()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
            HuntingRecord {
                id: HuntingRecordId(Uuid::new_v4()),
                user_id: UserId(user_id),
                hunting_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
                location: Some("別の猟場".to_string()),
                is_planned: true,
                notes: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
        ];

        let result = service.generate_hunting_records_csv(records).await;

        assert!(result.is_ok());
        let csv = result.unwrap();
        assert!(csv.contains("出猟日,場所,種別,備考"));
        assert!(csv.contains("テスト猟場"));
        assert!(csv.contains("実績"));
        assert!(csv.contains("予定"));
    }

    #[test]
    fn test_escape_csv_field() {
        assert_eq!(escape_csv_field("normal"), "normal");
        assert_eq!(escape_csv_field("contains,comma"), "\"contains,comma\"");
        assert_eq!(escape_csv_field("contains\"quote"), "\"contains\"\"quote\"");
        assert_eq!(
            escape_csv_field("contains\nnewline"),
            "\"contains\nnewline\""
        );
    }
}
