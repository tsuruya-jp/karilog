use application::usecases::AmmunitionStockUsecases;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use crate::middleware::auth_middleware::AuthenticatedUser;
use domain::value_objects::AmmunitionTypeId;
use shared::error::AppError;
use std::sync::Arc;

/// 在庫一覧を取得
pub async fn list_ammunition_stocks(
    State(ammunition_stock_usecases): State<Arc<AmmunitionStockUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let response = ammunition_stock_usecases.list_stocks(user_id).await?;

    Ok(Json(response))
}

/// 特定の実包種別の在庫を取得
pub async fn get_ammunition_stock(
    State(ammunition_stock_usecases): State<Arc<AmmunitionStockUsecases>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(ammunition_type_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ammunition_type_id = AmmunitionTypeId::from_str(&ammunition_type_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid ammunition type ID: {}", e)))?;

    let response = ammunition_stock_usecases
        .get_stock(user_id, ammunition_type_id)
        .await?;

    Ok(Json(response))
}
