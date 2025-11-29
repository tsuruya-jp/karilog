/// レポート生成サービスのトレイト
use crate::entities::{AmmunitionPurchase, AmmunitionUsage, HuntingRecord};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

/// PDF生成サービスのトレイト
#[async_trait]
pub trait PdfReportService: Send + Sync {
    /// 実包管理帳簿PDFを生成
    async fn generate_ammunition_ledger_pdf(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        purchases: Vec<AmmunitionPurchase>,
        usages: Vec<AmmunitionUsage>,
    ) -> Result<Vec<u8>>;

    /// 出猟サマリーPDFを生成
    async fn generate_hunting_summary_pdf(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        records: Vec<HuntingRecord>,
    ) -> Result<Vec<u8>>;
}

/// CSV生成サービスのトレイト
#[async_trait]
pub trait CsvReportService: Send + Sync {
    /// 実包購入記録をCSV形式で出力
    async fn generate_ammunition_purchases_csv(
        &self,
        purchases: Vec<AmmunitionPurchase>,
    ) -> Result<String>;

    /// 実包使用記録をCSV形式で出力
    async fn generate_ammunition_usages_csv(&self, usages: Vec<AmmunitionUsage>) -> Result<String>;

    /// 出猟記録をCSV形式で出力
    async fn generate_hunting_records_csv(&self, records: Vec<HuntingRecord>) -> Result<String>;
}
