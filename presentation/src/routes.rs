// ルーティング設定
use crate::handlers::auth_handlers::{
    delete_account, get_user_info, health_check, login, logout, refresh_token, register,
    request_password_reset, resend_verification_email, reset_password, update_user_info,
    verify_email, AuthState,
};
use axum::{
    routing::{get, patch, post},
    Router,
};
use std::sync::Arc;

/// ルーターを作成
pub fn create_router(
    auth_state: AuthState,
    jwt_service: Arc<infrastructure::services::JwtService>,
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

    // メインルーター
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1/auth", auth_routes)
}
