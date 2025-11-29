/// レポート出力ハンドラー
use application::dto::{CsvReportResponse, ReportRequest};
use application::usecases::ReportUsecases;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::Json;
use shared::error::AppError;
use std::sync::Arc;

use crate::middleware::AuthenticatedUser;

/// 実包管理帳簿PDFを生成
pub async fn generate_ammunition_ledger_pdf(
    State(report_usecases): State<Arc<ReportUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<ReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = report_usecases
        .generate_ammunition_ledger_pdf(user_id, request)
        .await?;

    Ok(Json(response))
}

/// 出猟サマリーPDFを生成
pub async fn generate_hunting_summary_pdf(
    State(report_usecases): State<Arc<ReportUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<ReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = report_usecases
        .generate_hunting_summary_pdf(user_id, request)
        .await?;

    Ok(Json(response))
}

/// 実包購入記録CSVを生成
pub async fn generate_ammunition_purchases_csv(
    State(report_usecases): State<Arc<ReportUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<ReportRequest>,
) -> Result<Response, AppError> {
    let response = report_usecases
        .generate_ammunition_purchases_csv(user_id, request)
        .await?;

    Ok(create_csv_response(response))
}

/// 実包使用記録CSVを生成
pub async fn generate_ammunition_usages_csv(
    State(report_usecases): State<Arc<ReportUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<ReportRequest>,
) -> Result<Response, AppError> {
    let response = report_usecases
        .generate_ammunition_usages_csv(user_id, request)
        .await?;

    Ok(create_csv_response(response))
}

/// 出猟記録CSVを生成
pub async fn generate_hunting_records_csv(
    State(report_usecases): State<Arc<ReportUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<ReportRequest>,
) -> Result<Response, AppError> {
    let response = report_usecases
        .generate_hunting_records_csv(user_id, request)
        .await?;

    Ok(create_csv_response(response))
}

/// CSVレスポンスを作成
fn create_csv_response(csv_response: CsvReportResponse) -> Response {
    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", csv_response.filename),
        )
        .body(csv_response.csv_data.into())
        .unwrap()
}
