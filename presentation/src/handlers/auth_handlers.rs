use application::dto::{
    DeleteAccountRequest, LoginRequest, LogoutRequest, RefreshTokenRequest, RegisterRequest,
    RequestPasswordResetRequest, ResendVerificationEmailRequest, ResetPasswordRequest,
    UpdateUserInfoRequest, VerifyEmailRequest,
};
use application::usecases::AuthUseCases;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use infrastructure::persistence::PostgresUserRepository;
use infrastructure::services::ConsoleEmailService;
use serde_json::json;
use shared::error::AppError;
use std::sync::Arc;
use validator::Validate;

/// 認証ハンドラで使用する共有状態
pub type AuthState = AppAuthState;

#[derive(Clone)]
pub struct AppAuthState {
    pub auth_usecases: Arc<AuthUseCases<PostgresUserRepository, ConsoleEmailService>>,
}

/// ユーザー登録ハンドラ
pub async fn register(
    State(state): State<AuthState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.register(request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// ログインハンドラ
pub async fn login(
    State(state): State<AuthState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.login(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// ログアウトハンドラ
pub async fn logout(
    State(state): State<AuthState>,
    Json(request): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    // ユースケースを実行
    let response = state.auth_usecases.logout(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// トークンリフレッシュハンドラ
pub async fn refresh_token(
    State(state): State<AuthState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    // ユースケースを実行
    let response = state.auth_usecases.refresh_token(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// メール確認ハンドラ
pub async fn verify_email(
    State(state): State<AuthState>,
    Json(request): Json<VerifyEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    // ユースケースを実行
    let response = state.auth_usecases.verify_email(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// パスワードリセットリクエストハンドラ
pub async fn request_password_reset(
    State(state): State<AuthState>,
    Json(request): Json<RequestPasswordResetRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.request_password_reset(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// パスワードリセットハンドラ
pub async fn reset_password(
    State(state): State<AuthState>,
    Json(request): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.reset_password(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// ユーザー情報取得ハンドラ（認証が必要）
pub async fn get_user_info(
    State(state): State<AuthState>,
    crate::middleware::auth_middleware::AuthenticatedUser(user_id): crate::middleware::auth_middleware::AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // ユースケースを実行
    let response = state.auth_usecases.get_user_info(&user_id).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// 確認メール再送信ハンドラ
pub async fn resend_verification_email(
    State(state): State<AuthState>,
    Json(request): Json<ResendVerificationEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.resend_verification_email(request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// ユーザー情報更新ハンドラ（認証が必要）
pub async fn update_user_info(
    State(state): State<AuthState>,
    crate::middleware::auth_middleware::AuthenticatedUser(user_id): crate::middleware::auth_middleware::AuthenticatedUser,
    Json(request): Json<UpdateUserInfoRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.update_user_info(&user_id, request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// アカウント削除ハンドラ（認証が必要）
pub async fn delete_account(
    State(state): State<AuthState>,
    crate::middleware::auth_middleware::AuthenticatedUser(user_id): crate::middleware::auth_middleware::AuthenticatedUser,
    Json(request): Json<DeleteAccountRequest>,
) -> Result<impl IntoResponse, AppError> {
    // リクエストのバリデーション
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // ユースケースを実行
    let response = state.auth_usecases.delete_account(&user_id, request).await?;

    Ok((StatusCode::OK, Json(response)))
}

/// ヘルスチェックハンドラ
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}
