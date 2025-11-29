use crate::middleware::auth_middleware::AuthenticatedUser;
use application::dto::{
    CreateHuntingRecordRequest, ListHuntingRecordsQuery, UpdateHuntingRecordRequest,
};
use application::usecases::HuntingRecordUsecases;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::value_objects::HuntingRecordId;
use shared::error::AppError;
use std::sync::Arc;

/// 出猟記録を作成
pub async fn create_hunting_record(
    State(hunting_record_usecases): State<Arc<HuntingRecordUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateHuntingRecordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = hunting_record_usecases
        .create_hunting_record(user_id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// 出猟記録を取得
pub async fn get_hunting_record(
    State(hunting_record_usecases): State<Arc<HuntingRecordUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(record_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let record_id = HuntingRecordId::from_str(&record_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid hunting record ID: {}", e)))?;

    let response = hunting_record_usecases
        .get_hunting_record(user_id, record_id)
        .await?;

    Ok(Json(response))
}

/// 出猟記録一覧を取得
pub async fn list_hunting_records(
    State(hunting_record_usecases): State<Arc<HuntingRecordUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<ListHuntingRecordsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let response = hunting_record_usecases
        .list_hunting_records(user_id, query)
        .await?;

    Ok(Json(response))
}

/// 出猟記録を更新
pub async fn update_hunting_record(
    State(hunting_record_usecases): State<Arc<HuntingRecordUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(record_id): Path<String>,
    Json(request): Json<UpdateHuntingRecordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let record_id = HuntingRecordId::from_str(&record_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid hunting record ID: {}", e)))?;

    let response = hunting_record_usecases
        .update_hunting_record(user_id, record_id, request)
        .await?;

    Ok(Json(response))
}

/// 出猟記録を削除
pub async fn delete_hunting_record(
    State(hunting_record_usecases): State<Arc<HuntingRecordUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(record_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let record_id = HuntingRecordId::from_str(&record_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid hunting record ID: {}", e)))?;

    hunting_record_usecases
        .delete_hunting_record(user_id, record_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// 出猟統計を取得
pub async fn get_statistics(
    State(hunting_record_usecases): State<Arc<HuntingRecordUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = hunting_record_usecases.get_statistics(user_id).await?;

    Ok(Json(response))
}
