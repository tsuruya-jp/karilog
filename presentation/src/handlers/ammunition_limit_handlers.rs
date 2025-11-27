use application::dto::{CreateAmmunitionLimitRequest, UpdateAmmunitionLimitRequest};
use application::usecases::AmmunitionLimitUsecases;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::middleware::auth_middleware::AuthenticatedUser;
use domain::value_objects::AmmunitionLimitId;
use shared::error::AppError;
use std::sync::Arc;

pub async fn create_ammunition_limit(
    State(ammunition_limit_usecases): State<Arc<AmmunitionLimitUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateAmmunitionLimitRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_limit_usecases
        .create_ammunition_limit(user_id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_ammunition_limit(
    State(ammunition_limit_usecases): State<Arc<AmmunitionLimitUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_limit_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_limit_id = AmmunitionLimitId::from_str(&ammunition_limit_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition limit ID: {}", e)))?;

    let response = ammunition_limit_usecases
        .get_ammunition_limit(user_id, ammunition_limit_id)
        .await?;

    Ok(Json(response))
}

pub async fn list_ammunition_limits(
    State(ammunition_limit_usecases): State<Arc<AmmunitionLimitUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_limit_usecases
        .list_ammunition_limits(user_id)
        .await?;

    Ok(Json(response))
}

pub async fn update_ammunition_limit(
    State(ammunition_limit_usecases): State<Arc<AmmunitionLimitUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_limit_id): Path<String>,
    Json(request): Json<UpdateAmmunitionLimitRequest>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_limit_id = AmmunitionLimitId::from_str(&ammunition_limit_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition limit ID: {}", e)))?;

    let response = ammunition_limit_usecases
        .update_ammunition_limit(user_id, ammunition_limit_id, request)
        .await?;

    Ok(Json(response))
}

pub async fn delete_ammunition_limit(
    State(ammunition_limit_usecases): State<Arc<AmmunitionLimitUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_limit_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_limit_id = AmmunitionLimitId::from_str(&ammunition_limit_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition limit ID: {}", e)))?;

    ammunition_limit_usecases
        .delete_ammunition_limit(user_id, ammunition_limit_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
