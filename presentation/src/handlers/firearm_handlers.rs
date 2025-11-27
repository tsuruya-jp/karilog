use application::dto::{CreateFirearmRequest, UpdateFirearmRequest};
use application::usecases::FirearmUsecases;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::middleware::auth_middleware::AuthenticatedUser;
use domain::value_objects::FirearmId;
use shared::error::AppError;
use std::sync::Arc;

/// 銃砲を作成
pub async fn create_firearm(
    State(firearm_usecases): State<Arc<FirearmUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateFirearmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = firearm_usecases
        .create_firearm(user_id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// 銃砲を取得
pub async fn get_firearm(
    State(firearm_usecases): State<Arc<FirearmUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(firearm_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let firearm_id = FirearmId::from_str(&firearm_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid firearm ID: {}", e)))?;

    let response = firearm_usecases.get_firearm(user_id, firearm_id).await?;

    Ok(Json(response))
}

/// 銃砲一覧を取得
pub async fn list_firearms(
    State(firearm_usecases): State<Arc<FirearmUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = firearm_usecases.list_firearms(user_id).await?;

    Ok(Json(response))
}

/// 銃砲を更新
pub async fn update_firearm(
    State(firearm_usecases): State<Arc<FirearmUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(firearm_id): Path<String>,
    Json(request): Json<UpdateFirearmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let firearm_id = FirearmId::from_str(&firearm_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid firearm ID: {}", e)))?;

    let response = firearm_usecases
        .update_firearm(user_id, firearm_id, request)
        .await?;

    Ok(Json(response))
}

/// 銃砲を削除
pub async fn delete_firearm(
    State(firearm_usecases): State<Arc<FirearmUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(firearm_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let firearm_id = FirearmId::from_str(&firearm_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid firearm ID: {}", e)))?;

    firearm_usecases.delete_firearm(user_id, firearm_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
