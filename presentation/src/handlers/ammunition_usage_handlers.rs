use crate::middleware::auth_middleware::AuthenticatedUser;
use application::dto::{CreateAmmunitionUsageRequest, UpdateAmmunitionUsageRequest};
use application::usecases::AmmunitionUsageUsecases;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::value_objects::AmmunitionUsageId;
use shared::error::AppError;
use std::sync::Arc;

pub async fn create_ammunition_usage(
    State(ammunition_usage_usecases): State<Arc<AmmunitionUsageUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateAmmunitionUsageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_usage_usecases
        .create_ammunition_usage(user_id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_ammunition_usage(
    State(ammunition_usage_usecases): State<Arc<AmmunitionUsageUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_usage_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_usage_id = AmmunitionUsageId::from_str(&ammunition_usage_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition usage ID: {}", e)))?;

    let response = ammunition_usage_usecases
        .get_ammunition_usage(user_id, ammunition_usage_id)
        .await?;

    Ok(Json(response))
}

pub async fn list_ammunition_usages(
    State(ammunition_usage_usecases): State<Arc<AmmunitionUsageUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_usage_usecases
        .list_ammunition_usages(user_id)
        .await?;

    Ok(Json(response))
}

pub async fn update_ammunition_usage(
    State(ammunition_usage_usecases): State<Arc<AmmunitionUsageUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_usage_id): Path<String>,
    Json(request): Json<UpdateAmmunitionUsageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_usage_id = AmmunitionUsageId::from_str(&ammunition_usage_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition usage ID: {}", e)))?;

    let response = ammunition_usage_usecases
        .update_ammunition_usage(user_id, ammunition_usage_id, request)
        .await?;

    Ok(Json(response))
}

pub async fn delete_ammunition_usage(
    State(ammunition_usage_usecases): State<Arc<AmmunitionUsageUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_usage_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_usage_id = AmmunitionUsageId::from_str(&ammunition_usage_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition usage ID: {}", e)))?;

    ammunition_usage_usecases
        .delete_ammunition_usage(user_id, ammunition_usage_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
