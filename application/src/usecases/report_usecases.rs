/// レポート生成ユースケース
use crate::dto::{CsvReportResponse, PdfReportResponse, ReportRequest};
use anyhow::{Context, Result};
use base64::Engine;
use domain::repositories::{
    AmmunitionPurchaseRepository, AmmunitionUsageRepository, HuntingRecordRepository,
};
use domain::services::{CsvReportService, PdfReportService};
use domain::value_objects::UserId;
use std::sync::Arc;

/// レポート生成ユースケース
pub struct ReportUsecases {
    ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
    ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
    hunting_record_repository: Arc<dyn HuntingRecordRepository>,
    pdf_report_service: Arc<dyn PdfReportService>,
    csv_report_service: Arc<dyn CsvReportService>,
}

impl ReportUsecases {
    pub fn new(
        ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
        ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
        hunting_record_repository: Arc<dyn HuntingRecordRepository>,
        pdf_report_service: Arc<dyn PdfReportService>,
        csv_report_service: Arc<dyn CsvReportService>,
    ) -> Self {
        Self {
            ammunition_purchase_repository,
            ammunition_usage_repository,
            hunting_record_repository,
            pdf_report_service,
            csv_report_service,
        }
    }

    /// 実包管理帳簿PDFを生成
    pub async fn generate_ammunition_ledger_pdf(
        &self,
        user_id: UserId,
        request: ReportRequest,
    ) -> Result<PdfReportResponse> {
        // 購入記録を取得
        let purchases = self
            .ammunition_purchase_repository
            .find_by_user_id_and_date_range(&user_id, request.start_date, request.end_date)
            .await
            .context("購入記録の取得に失敗しました")?;

        // 使用記録を取得
        let usages = self
            .ammunition_usage_repository
            .find_by_user_id_and_date_range(&user_id, request.start_date, request.end_date)
            .await
            .context("使用記録の取得に失敗しました")?;

        // PDFを生成
        let pdf_bytes = self
            .pdf_report_service
            .generate_ammunition_ledger_pdf(
                *user_id.as_uuid(),
                request.start_date,
                request.end_date,
                purchases,
                usages,
            )
            .await
            .context("PDFの生成に失敗しました")?;

        // Base64エンコード
        let pdf_data = base64::engine::general_purpose::STANDARD.encode(&pdf_bytes);

        Ok(PdfReportResponse {
            pdf_data,
            filename: format!(
                "ammunition_ledger_{}_to_{}.pdf",
                request.start_date.format("%Y%m%d"),
                request.end_date.format("%Y%m%d")
            ),
        })
    }

    /// 出猟サマリーPDFを生成
    pub async fn generate_hunting_summary_pdf(
        &self,
        user_id: UserId,
        request: ReportRequest,
    ) -> Result<PdfReportResponse> {
        // 出猟記録を取得
        let records = self
            .hunting_record_repository
            .find_by_user_and_date_range(&user_id, request.start_date, request.end_date)
            .await
            .context("出猟記録の取得に失敗しました")?;

        // PDFを生成
        let pdf_bytes = self
            .pdf_report_service
            .generate_hunting_summary_pdf(
                *user_id.as_uuid(),
                request.start_date,
                request.end_date,
                records,
            )
            .await
            .context("PDFの生成に失敗しました")?;

        // Base64エンコード
        let pdf_data = base64::engine::general_purpose::STANDARD.encode(&pdf_bytes);

        Ok(PdfReportResponse {
            pdf_data,
            filename: format!(
                "hunting_summary_{}_to_{}.pdf",
                request.start_date.format("%Y%m%d"),
                request.end_date.format("%Y%m%d")
            ),
        })
    }

    /// 実包購入記録CSVを生成
    pub async fn generate_ammunition_purchases_csv(
        &self,
        user_id: UserId,
        request: ReportRequest,
    ) -> Result<CsvReportResponse> {
        // 購入記録を取得
        let purchases = self
            .ammunition_purchase_repository
            .find_by_user_id_and_date_range(&user_id, request.start_date, request.end_date)
            .await
            .context("購入記録の取得に失敗しました")?;

        // CSVを生成
        let csv_data = self
            .csv_report_service
            .generate_ammunition_purchases_csv(purchases)
            .await
            .context("CSVの生成に失敗しました")?;

        Ok(CsvReportResponse {
            csv_data,
            filename: format!(
                "ammunition_purchases_{}_to_{}.csv",
                request.start_date.format("%Y%m%d"),
                request.end_date.format("%Y%m%d")
            ),
        })
    }

    /// 実包使用記録CSVを生成
    pub async fn generate_ammunition_usages_csv(
        &self,
        user_id: UserId,
        request: ReportRequest,
    ) -> Result<CsvReportResponse> {
        // 使用記録を取得
        let usages = self
            .ammunition_usage_repository
            .find_by_user_id_and_date_range(&user_id, request.start_date, request.end_date)
            .await
            .context("使用記録の取得に失敗しました")?;

        // CSVを生成
        let csv_data = self
            .csv_report_service
            .generate_ammunition_usages_csv(usages)
            .await
            .context("CSVの生成に失敗しました")?;

        Ok(CsvReportResponse {
            csv_data,
            filename: format!(
                "ammunition_usages_{}_to_{}.csv",
                request.start_date.format("%Y%m%d"),
                request.end_date.format("%Y%m%d")
            ),
        })
    }

    /// 出猟記録CSVを生成
    pub async fn generate_hunting_records_csv(
        &self,
        user_id: UserId,
        request: ReportRequest,
    ) -> Result<CsvReportResponse> {
        // 出猟記録を取得
        let records = self
            .hunting_record_repository
            .find_by_user_and_date_range(&user_id, request.start_date, request.end_date)
            .await
            .context("出猟記録の取得に失敗しました")?;

        // CSVを生成
        let csv_data = self
            .csv_report_service
            .generate_hunting_records_csv(records)
            .await
            .context("CSVの生成に失敗しました")?;

        Ok(CsvReportResponse {
            csv_data,
            filename: format!(
                "hunting_records_{}_to_{}.csv",
                request.start_date.format("%Y%m%d"),
                request.end_date.format("%Y%m%d")
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use domain::entities::{AmmunitionPurchase, AmmunitionUsage, HuntingRecord};
    use domain::value_objects::{
        AmmunitionPurchaseId, AmmunitionTypeId, AmmunitionUsageId, FirearmId, HuntingRecordId,
    };
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;

    mock! {
        AmmunitionPurchaseRepo {}
        #[async_trait::async_trait]
        impl AmmunitionPurchaseRepository for AmmunitionPurchaseRepo {
            async fn create(&self, purchase: &AmmunitionPurchase) -> Result<AmmunitionPurchase>;
            async fn find_by_id(&self, id: AmmunitionPurchaseId) -> Result<Option<AmmunitionPurchase>>;
            async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<AmmunitionPurchase>>;
            async fn find_by_user_id_and_date_range(&self, user_id: UserId, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<AmmunitionPurchase>>;
            async fn update(&self, purchase: &AmmunitionPurchase) -> Result<AmmunitionPurchase>;
            async fn delete(&self, id: AmmunitionPurchaseId) -> Result<()>;
        }
    }

    mock! {
        AmmunitionUsageRepo {}
        #[async_trait::async_trait]
        impl AmmunitionUsageRepository for AmmunitionUsageRepo {
            async fn create(&self, usage: &AmmunitionUsage) -> Result<AmmunitionUsage>;
            async fn find_by_id(&self, id: AmmunitionUsageId) -> Result<Option<AmmunitionUsage>>;
            async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<AmmunitionUsage>>;
            async fn find_by_user_id_and_date_range(&self, user_id: UserId, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<AmmunitionUsage>>;
            async fn update(&self, usage: &AmmunitionUsage) -> Result<AmmunitionUsage>;
            async fn delete(&self, id: AmmunitionUsageId) -> Result<()>;
        }
    }

    mock! {
        HuntingRecordRepo {}
        #[async_trait::async_trait]
        impl HuntingRecordRepository for HuntingRecordRepo {
            async fn create(&self, record: &HuntingRecord) -> Result<HuntingRecord>;
            async fn find_by_id(&self, id: HuntingRecordId) -> Result<Option<HuntingRecord>>;
            async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<HuntingRecord>>;
            async fn find_by_user_id_and_date_range(&self, user_id: UserId, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<HuntingRecord>>;
            async fn update(&self, record: &HuntingRecord) -> Result<HuntingRecord>;
            async fn delete(&self, id: HuntingRecordId) -> Result<()>;
        }
    }

    mock! {
        PdfService {}
        #[async_trait::async_trait]
        impl PdfReportService for PdfService {
            async fn generate_ammunition_ledger_pdf(&self, user_id: Uuid, start_date: NaiveDate, end_date: NaiveDate, purchases: Vec<AmmunitionPurchase>, usages: Vec<AmmunitionUsage>) -> Result<Vec<u8>>;
            async fn generate_hunting_summary_pdf(&self, user_id: Uuid, start_date: NaiveDate, end_date: NaiveDate, records: Vec<HuntingRecord>) -> Result<Vec<u8>>;
        }
    }

    mock! {
        CsvService {}
        #[async_trait::async_trait]
        impl CsvReportService for CsvService {
            async fn generate_ammunition_purchases_csv(&self, purchases: Vec<AmmunitionPurchase>) -> Result<String>;
            async fn generate_ammunition_usages_csv(&self, usages: Vec<AmmunitionUsage>) -> Result<String>;
            async fn generate_hunting_records_csv(&self, records: Vec<HuntingRecord>) -> Result<String>;
        }
    }

    #[tokio::test]
    async fn test_generate_ammunition_ledger_pdf() {
        let mut mock_purchase_repo = MockAmmunitionPurchaseRepo::new();
        let mut mock_usage_repo = MockAmmunitionUsageRepo::new();
        let mock_hunting_repo = MockHuntingRecordRepo::new();
        let mut mock_pdf_service = MockPdfService::new();
        let mock_csv_service = MockCsvService::new();

        let user_id = UserId(Uuid::new_v4());
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        mock_purchase_repo
            .expect_find_by_user_id_and_date_range()
            .returning(|_, _, _| Ok(vec![]));

        mock_usage_repo
            .expect_find_by_user_id_and_date_range()
            .returning(|_, _, _| Ok(vec![]));

        mock_pdf_service
            .expect_generate_ammunition_ledger_pdf()
            .returning(|_, _, _, _, _| Ok(vec![0x25, 0x50, 0x44, 0x46])); // "%PDF"

        let usecases = ReportUsecases::new(
            Arc::new(mock_purchase_repo),
            Arc::new(mock_usage_repo),
            Arc::new(mock_hunting_repo),
            Arc::new(mock_pdf_service),
            Arc::new(mock_csv_service),
        );

        let request = ReportRequest {
            start_date,
            end_date,
        };

        let result = usecases
            .generate_ammunition_ledger_pdf(user_id, request)
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.pdf_data.is_empty());
        assert!(response.filename.contains("ammunition_ledger"));
    }

    #[tokio::test]
    async fn test_generate_ammunition_purchases_csv() {
        let mut mock_purchase_repo = MockAmmunitionPurchaseRepo::new();
        let mock_usage_repo = MockAmmunitionUsageRepo::new();
        let mock_hunting_repo = MockHuntingRecordRepo::new();
        let mock_pdf_service = MockPdfService::new();
        let mut mock_csv_service = MockCsvService::new();

        let user_id = UserId(Uuid::new_v4());
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        mock_purchase_repo
            .expect_find_by_user_id_and_date_range()
            .returning(|_, _, _| Ok(vec![]));

        mock_csv_service
            .expect_generate_ammunition_purchases_csv()
            .returning(|_| Ok("購入日,実包種別ID,銃砲ID,販売店,数量,価格,備考\n".to_string()));

        let usecases = ReportUsecases::new(
            Arc::new(mock_purchase_repo),
            Arc::new(mock_usage_repo),
            Arc::new(mock_hunting_repo),
            Arc::new(mock_pdf_service),
            Arc::new(mock_csv_service),
        );

        let request = ReportRequest {
            start_date,
            end_date,
        };

        let result = usecases
            .generate_ammunition_purchases_csv(user_id, request)
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.csv_data.is_empty());
        assert!(response.filename.contains("ammunition_purchases"));
    }
}
