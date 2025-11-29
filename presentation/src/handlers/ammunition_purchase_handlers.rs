use crate::middleware::auth_middleware::AuthenticatedUser;
use application::dto::{CreateAmmunitionPurchaseRequest, UpdateAmmunitionPurchaseRequest};
use application::usecases::AmmunitionPurchaseUsecases;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::value_objects::AmmunitionPurchaseId;
use shared::error::AppError;
use std::sync::Arc;

pub async fn create_ammunition_purchase(
    State(ammunition_purchase_usecases): State<Arc<AmmunitionPurchaseUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateAmmunitionPurchaseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_purchase_usecases
        .create_ammunition_purchase(user_id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_ammunition_purchase(
    State(ammunition_purchase_usecases): State<Arc<AmmunitionPurchaseUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_purchase_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_purchase_id = AmmunitionPurchaseId::from_str(&ammunition_purchase_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition purchase ID: {}", e)))?;

    let response = ammunition_purchase_usecases
        .get_ammunition_purchase(user_id, ammunition_purchase_id)
        .await?;

    Ok(Json(response))
}

pub async fn list_ammunition_purchases(
    State(ammunition_purchase_usecases): State<Arc<AmmunitionPurchaseUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_purchase_usecases
        .list_ammunition_purchases(user_id)
        .await?;

    Ok(Json(response))
}

pub async fn update_ammunition_purchase(
    State(ammunition_purchase_usecases): State<Arc<AmmunitionPurchaseUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_purchase_id): Path<String>,
    Json(request): Json<UpdateAmmunitionPurchaseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_purchase_id = AmmunitionPurchaseId::from_str(&ammunition_purchase_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition purchase ID: {}", e)))?;

    let response = ammunition_purchase_usecases
        .update_ammunition_purchase(user_id, ammunition_purchase_id, request)
        .await?;

    Ok(Json(response))
}

pub async fn delete_ammunition_purchase(
    State(ammunition_purchase_usecases): State<Arc<AmmunitionPurchaseUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_purchase_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_purchase_id = AmmunitionPurchaseId::from_str(&ammunition_purchase_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition purchase ID: {}", e)))?;

    ammunition_purchase_usecases
        .delete_ammunition_purchase(user_id, ammunition_purchase_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
