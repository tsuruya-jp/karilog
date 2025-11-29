/// PDF生成サービスの実装
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use domain::entities::{AmmunitionPurchase, AmmunitionUsage, HuntingRecord};
use domain::services::PdfReportService;
use genpdf::elements::{Paragraph, TableLayout, Text};
use genpdf::style::Style;
use genpdf::{Alignment, Document, Element};
use uuid::Uuid;

pub struct PdfService;

impl PdfService {
    pub fn new() -> Self {
        Self
    }

    /// PDFドキュメントを初期化
    fn init_document(title: &str) -> Result<Document> {
        // NOTE: 日本語フォント対応のため、./fonts ディレクトリに NotoSansJP フォントファイルを配置してください
        // 例: ./fonts/NotoSansJP-Regular.ttf
        // ダウンロード先: https://fonts.google.com/noto/specimen/Noto+Sans+JP
        //
        // フォントファイルが見つからない場合は、エラーを返します
        let font_family = genpdf::fonts::from_files("./fonts", "NotoSansJP", None)
            .context("フォントの読み込みに失敗しました。./fonts ディレクトリに NotoSansJP フォントを配置してください")?;

        let mut doc = Document::new(font_family);
        doc.set_title(title);
        doc.set_minimal_conformance();

        Ok(doc)
    }

    /// 日付範囲のヘッダーを追加
    fn add_date_range_header(
        doc: &mut Document,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<()> {
        doc.push(
            Paragraph::new(format!(
                "期間: {} ～ {}",
                start_date.format("%Y年%m月%d日"),
                end_date.format("%Y年%m月%d日")
            ))
            .styled(Style::new().bold()),
        );
        doc.push(genpdf::elements::Break::new(1.0));
        Ok(())
    }
}

impl Default for PdfService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PdfReportService for PdfService {
    async fn generate_ammunition_ledger_pdf(
        &self,
        _user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        purchases: Vec<AmmunitionPurchase>,
        usages: Vec<AmmunitionUsage>,
    ) -> Result<Vec<u8>> {
        let mut doc = Self::init_document("実包管理帳簿")?;

        // タイトル
        doc.push(
            Paragraph::new("実包管理帳簿")
                .aligned(Alignment::Center)
                .styled(Style::new().bold().with_font_size(16)),
        );
        doc.push(genpdf::elements::Break::new(1.0));

        // 期間表示
        Self::add_date_range_header(&mut doc, start_date, end_date)?;

        // 購入記録セクション
        doc.push(Paragraph::new("購入記録").styled(Style::new().bold().with_font_size(12)));
        doc.push(genpdf::elements::Break::new(0.5));

        if !purchases.is_empty() {
            let mut table = TableLayout::new(vec![1, 2, 2, 1, 1, 2]);
            table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

            // ヘッダー行
            table
                .row()
                .element(Paragraph::new("購入日").styled(Style::new().bold()))
                .element(Paragraph::new("実包種別ID").styled(Style::new().bold()))
                .element(Paragraph::new("販売店").styled(Style::new().bold()))
                .element(Paragraph::new("数量").styled(Style::new().bold()))
                .element(Paragraph::new("価格").styled(Style::new().bold()))
                .element(Paragraph::new("備考").styled(Style::new().bold()))
                .push()
                .context("テーブルヘッダーの追加に失敗しました")?;

            // データ行
            for purchase in purchases {
                table
                    .row()
                    .element(Text::new(
                        purchase.purchase_date.format("%Y-%m-%d").to_string(),
                    ))
                    .element(Text::new(purchase.ammunition_type_id.to_string()))
                    .element(Text::new(&purchase.supplier))
                    .element(Text::new(purchase.quantity.to_string()))
                    .element(Text::new(
                        purchase
                            .price
                            .map(|p| p.to_string())
                            .unwrap_or_else(|| "-".to_string()),
                    ))
                    .element(Text::new(
                        purchase.notes.as_deref().unwrap_or("-").to_string(),
                    ))
                    .push()
                    .context("テーブル行の追加に失敗しました")?;
            }

            doc.push(table);
        } else {
            doc.push(Paragraph::new("購入記録はありません"));
        }

        doc.push(genpdf::elements::Break::new(2.0));

        // 使用記録セクション
        doc.push(Paragraph::new("使用記録").styled(Style::new().bold().with_font_size(12)));
        doc.push(genpdf::elements::Break::new(0.5));

        if !usages.is_empty() {
            let mut table = TableLayout::new(vec![1, 2, 2, 1, 2]);
            table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

            // ヘッダー行
            table
                .row()
                .element(Paragraph::new("使用日").styled(Style::new().bold()))
                .element(Paragraph::new("実包種別ID").styled(Style::new().bold()))
                .element(Paragraph::new("場所").styled(Style::new().bold()))
                .element(Paragraph::new("使用数").styled(Style::new().bold()))
                .element(Paragraph::new("備考").styled(Style::new().bold()))
                .push()
                .context("テーブルヘッダーの追加に失敗しました")?;

            // データ行
            for usage in usages {
                table
                    .row()
                    .element(Text::new(usage.usage_date.format("%Y-%m-%d").to_string()))
                    .element(Text::new(usage.ammunition_type_id.to_string()))
                    .element(Text::new(
                        usage.location.as_deref().unwrap_or("-").to_string(),
                    ))
                    .element(Text::new(usage.quantity_used.to_string()))
                    .element(Text::new(usage.notes.as_deref().unwrap_or("-").to_string()))
                    .push()
                    .context("テーブル行の追加に失敗しました")?;
            }

            doc.push(table);
        } else {
            doc.push(Paragraph::new("使用記録はありません"));
        }

        // PDFをバイト列として出力
        let mut bytes = Vec::new();
        doc.render(&mut bytes)
            .context("PDFのレンダリングに失敗しました")?;

        Ok(bytes)
    }

    async fn generate_hunting_summary_pdf(
        &self,
        _user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        records: Vec<HuntingRecord>,
    ) -> Result<Vec<u8>> {
        let mut doc = Self::init_document("出猟サマリー")?;

        // タイトル
        doc.push(
            Paragraph::new("出猟サマリー")
                .aligned(Alignment::Center)
                .styled(Style::new().bold().with_font_size(16)),
        );
        doc.push(genpdf::elements::Break::new(1.0));

        // 期間表示
        Self::add_date_range_header(&mut doc, start_date, end_date)?;

        // 統計情報
        let total_count = records.len();
        let actual_count = records.iter().filter(|r| !r.is_planned).count();
        let planned_count = records.iter().filter(|r| r.is_planned).count();

        doc.push(Paragraph::new(format!("総出猟回数: {}", total_count)));
        doc.push(Paragraph::new(format!("実績: {}", actual_count)));
        doc.push(Paragraph::new(format!("予定: {}", planned_count)));
        doc.push(genpdf::elements::Break::new(2.0));

        // 出猟記録テーブル
        doc.push(Paragraph::new("出猟記録一覧").styled(Style::new().bold().with_font_size(12)));
        doc.push(genpdf::elements::Break::new(0.5));

        if !records.is_empty() {
            let mut table = TableLayout::new(vec![1, 2, 1, 3]);
            table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

            // ヘッダー行
            table
                .row()
                .element(Paragraph::new("日付").styled(Style::new().bold()))
                .element(Paragraph::new("場所").styled(Style::new().bold()))
                .element(Paragraph::new("種別").styled(Style::new().bold()))
                .element(Paragraph::new("備考").styled(Style::new().bold()))
                .push()
                .context("テーブルヘッダーの追加に失敗しました")?;

            // データ行
            for record in records {
                table
                    .row()
                    .element(Text::new(
                        record.hunting_date.format("%Y-%m-%d").to_string(),
                    ))
                    .element(Text::new(
                        record.location.as_deref().unwrap_or("-").to_string(),
                    ))
                    .element(Text::new(if record.is_planned {
                        "予定"
                    } else {
                        "実績"
                    }))
                    .element(Text::new(
                        record.notes.as_deref().unwrap_or("-").to_string(),
                    ))
                    .push()
                    .context("テーブル行の追加に失敗しました")?;
            }

            doc.push(table);
        } else {
            doc.push(Paragraph::new("出猟記録はありません"));
        }

        // PDFをバイト列として出力
        let mut bytes = Vec::new();
        doc.render(&mut bytes)
            .context("PDFのレンダリングに失敗しました")?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use domain::value_objects::{
        AmmunitionPurchaseId, AmmunitionTypeId, AmmunitionUsageId, FirearmId, HuntingRecordId,
        UserId,
    };

    #[tokio::test]
    async fn test_generate_ammunition_ledger_pdf() {
        let service = PdfService::new();
        let user_id = Uuid::new_v4();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let purchases = vec![AmmunitionPurchase {
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
        }];

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

        let result = service
            .generate_ammunition_ledger_pdf(user_id, start_date, end_date, purchases, usages)
            .await;

        assert!(result.is_ok());
        let pdf_bytes = result.unwrap();
        assert!(!pdf_bytes.is_empty());
    }

    #[tokio::test]
    async fn test_generate_hunting_summary_pdf() {
        let service = PdfService::new();
        let user_id = Uuid::new_v4();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let records = vec![HuntingRecord {
            id: HuntingRecordId(Uuid::new_v4()),
            user_id: UserId(user_id),
            hunting_date: NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            location: Some("テスト猟場".to_string()),
            is_planned: false,
            notes: Some("テスト出猟".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }];

        let result = service
            .generate_hunting_summary_pdf(user_id, start_date, end_date, records)
            .await;

        assert!(result.is_ok());
        let pdf_bytes = result.unwrap();
        assert!(!pdf_bytes.is_empty());
    }
}
