/// レポート出力用DTO
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// レポート生成リクエスト
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_date_range", skip_on_field_errors = false))]
pub struct ReportRequest {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

/// 日付範囲のバリデーション
fn validate_date_range(request: &ReportRequest) -> Result<(), validator::ValidationError> {
    if request.start_date > request.end_date {
        return Err(validator::ValidationError::new(
            "start_date must be before or equal to end_date",
        ));
    }
    Ok(())
}

/// PDFレポートレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfReportResponse {
    /// PDF データ（Base64エンコード）
    pub pdf_data: String,
    /// ファイル名
    pub filename: String,
}

/// CSVレポートレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvReportResponse {
    /// CSV データ
    pub csv_data: String,
    /// ファイル名
    pub filename: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_report_request() {
        let request = ReportRequest {
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_invalid_report_request() {
        let request = ReportRequest {
            start_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_same_date_report_request() {
        let request = ReportRequest {
            start_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
        };

        assert!(request.validate().is_ok());
    }
}
