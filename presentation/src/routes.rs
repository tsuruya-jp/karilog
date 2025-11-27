// ルーティング設定
use crate::handlers::auth_handlers::{
    delete_account, get_user_info, health_check, login, logout, refresh_token, register,
    request_password_reset, resend_verification_email, reset_password, update_user_info,
    verify_email, AuthState,
};
// Phase2: 銃砲・実包管理
use crate::handlers::{
    create_ammunition_limit, create_ammunition_purchase, create_ammunition_type,
    create_ammunition_usage, create_firearm, delete_ammunition_limit, delete_ammunition_purchase,
    delete_ammunition_type, delete_ammunition_usage, delete_firearm, get_ammunition_limit,
    get_ammunition_purchase, get_ammunition_stock, get_ammunition_type, get_ammunition_usage,
    get_firearm, list_ammunition_limits, list_ammunition_purchases, list_ammunition_stocks,
    list_ammunition_types, list_ammunition_usages, list_firearms, update_ammunition_limit,
    update_ammunition_purchase, update_ammunition_type, update_ammunition_usage, update_firearm,
};
// Phase3: 出猟管理
use crate::handlers::{
    create_hunting_record, delete_hunting_record, get_hunting_record, get_statistics,
    list_hunting_records, update_hunting_record,
};
use application::usecases::{
    AmmunitionLimitUsecases, AmmunitionPurchaseUsecases, AmmunitionStockUsecases,
    AmmunitionTypeUsecases, AmmunitionUsageUsecases, FirearmUsecases, HuntingRecordUsecases,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

/// ルーターを作成
pub fn create_router(
    auth_state: AuthState,
    jwt_service: Arc<infrastructure::services::JwtService>,
    // Phase2のユースケース
    firearm_usecases: Arc<FirearmUsecases>,
    ammunition_type_usecases: Arc<AmmunitionTypeUsecases>,
    ammunition_limit_usecases: Arc<AmmunitionLimitUsecases>,
    ammunition_purchase_usecases: Arc<AmmunitionPurchaseUsecases>,
    ammunition_usage_usecases: Arc<AmmunitionUsageUsecases>,
    ammunition_stock_usecases: Arc<AmmunitionStockUsecases>,
    // Phase3のユースケース
    hunting_record_usecases: Arc<HuntingRecordUsecases>,
) -> Router {
    // 認証不要のルート
    let public_auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
        .route("/verify-email", post(verify_email))
        .route("/verify-email/resend", post(resend_verification_email))
        .route("/password/forgot", post(request_password_reset))
        .route("/password/reset", post(reset_password))
        .with_state(auth_state.clone());

    // 認証が必要なルート
    let protected_auth_routes = Router::new()
        .route("/me", get(get_user_info))
        .route("/me", patch(update_user_info))
        .route("/me/delete", post(delete_account))
        .with_state(auth_state)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // 認証ルートを統合
    let auth_routes = public_auth_routes.merge(protected_auth_routes);

    // Phase2: 銃砲ルート（認証必須）
    let firearm_routes = Router::new()
        .route("/", post(create_firearm))
        .route("/", get(list_firearms))
        .route("/:id", get(get_firearm))
        .route("/:id", patch(update_firearm))
        .route("/:id", delete(delete_firearm))
        .with_state(firearm_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // Phase2: 実包種別ルート（認証必須）
    let ammunition_type_routes = Router::new()
        .route("/", post(create_ammunition_type))
        .route("/", get(list_ammunition_types))
        .route("/:id", get(get_ammunition_type))
        .route("/:id", patch(update_ammunition_type))
        .route("/:id", delete(delete_ammunition_type))
        .with_state(ammunition_type_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // Phase2: 実包所持許可上限ルート（認証必須）
    let ammunition_limit_routes = Router::new()
        .route("/", post(create_ammunition_limit))
        .route("/", get(list_ammunition_limits))
        .route("/:id", get(get_ammunition_limit))
        .route("/:id", patch(update_ammunition_limit))
        .route("/:id", delete(delete_ammunition_limit))
        .with_state(ammunition_limit_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // Phase2: 実包購入記録ルート（認証必須）
    let ammunition_purchase_routes = Router::new()
        .route("/", post(create_ammunition_purchase))
        .route("/", get(list_ammunition_purchases))
        .route("/:id", get(get_ammunition_purchase))
        .route("/:id", patch(update_ammunition_purchase))
        .route("/:id", delete(delete_ammunition_purchase))
        .with_state(ammunition_purchase_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // Phase2: 実包使用記録ルート（認証必須）
    let ammunition_usage_routes = Router::new()
        .route("/", post(create_ammunition_usage))
        .route("/", get(list_ammunition_usages))
        .route("/:id", get(get_ammunition_usage))
        .route("/:id", patch(update_ammunition_usage))
        .route("/:id", delete(delete_ammunition_usage))
        .with_state(ammunition_usage_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // Phase2: 実包在庫ルート（認証必須）
    let ammunition_stock_routes = Router::new()
        .route("/", get(list_ammunition_stocks))
        .route("/:id", get(get_ammunition_stock))
        .with_state(ammunition_stock_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // Phase3: 出猟記録ルート（認証必須）
    let hunting_record_routes = Router::new()
        .route("/", post(create_hunting_record))
        .route("/", get(list_hunting_records))
        .route("/statistics", get(get_statistics))
        .route("/:id", get(get_hunting_record))
        .route("/:id", patch(update_hunting_record))
        .route("/:id", delete(delete_hunting_record))
        .with_state(hunting_record_usecases)
        .layer(axum::extract::Extension(jwt_service.clone()));

    // メインルーター
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1/auth", auth_routes)
        .nest("/api/v1/firearms", firearm_routes)
        .nest("/api/v1/ammunition-types", ammunition_type_routes)
        .nest("/api/v1/ammunition-limits", ammunition_limit_routes)
        .nest("/api/v1/ammunition-purchases", ammunition_purchase_routes)
        .nest("/api/v1/ammunition-usages", ammunition_usage_routes)
        .nest("/api/v1/ammunition-stock", ammunition_stock_routes)
        .nest("/api/v1/hunting-records", hunting_record_routes)
}
