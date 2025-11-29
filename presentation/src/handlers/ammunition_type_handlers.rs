use crate::middleware::auth_middleware::AuthenticatedUser;
use application::dto::{CreateAmmunitionTypeRequest, UpdateAmmunitionTypeRequest};
use application::usecases::AmmunitionTypeUsecases;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::value_objects::AmmunitionTypeId;
use shared::error::AppError;
use std::sync::Arc;

pub async fn create_ammunition_type(
    State(ammunition_type_usecases): State<Arc<AmmunitionTypeUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateAmmunitionTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_type_usecases
        .create_ammunition_type(user_id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_ammunition_type(
    State(ammunition_type_usecases): State<Arc<AmmunitionTypeUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_type_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_type_id = AmmunitionTypeId::from_str(&ammunition_type_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition type ID: {}", e)))?;

    let response = ammunition_type_usecases
        .get_ammunition_type(user_id, ammunition_type_id)
        .await?;

    Ok(Json(response))
}

pub async fn list_ammunition_types(
    State(ammunition_type_usecases): State<Arc<AmmunitionTypeUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_type_usecases
        .list_ammunition_types(user_id)
        .await?;

    Ok(Json(response))
}

pub async fn update_ammunition_type(
    State(ammunition_type_usecases): State<Arc<AmmunitionTypeUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_type_id): Path<String>,
    Json(request): Json<UpdateAmmunitionTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_type_id = AmmunitionTypeId::from_str(&ammunition_type_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition type ID: {}", e)))?;

    let response = ammunition_type_usecases
        .update_ammunition_type(user_id, ammunition_type_id, request)
        .await?;

    Ok(Json(response))
}

pub async fn delete_ammunition_type(
    State(ammunition_type_usecases): State<Arc<AmmunitionTypeUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_type_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_type_id = AmmunitionTypeId::from_str(&ammunition_type_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition type ID: {}", e)))?;

    ammunition_type_usecases
        .delete_ammunition_type(user_id, ammunition_type_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
