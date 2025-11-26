use serde::{Deserialize, Serialize};
use validator::Validate;

// ユーザー登録

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
}

// ログイン

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
}

// トークンリフレッシュ

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

// メール確認

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyEmailResponse {
    pub success: bool,
    pub message: String,
}

// パスワードリセットリクエスト

#[derive(Debug, Deserialize, Validate)]
pub struct RequestPasswordResetRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct RequestPasswordResetResponse {
    pub success: bool,
    pub message: String,
}

// パスワードリセット

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
}

// ログアウト

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub success: bool,
    pub message: String,
}

// ユーザー情報取得

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}

// 確認メール再送信

#[derive(Debug, Deserialize, Validate)]
pub struct ResendVerificationEmailRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ResendVerificationEmailResponse {
    pub success: bool,
    pub message: String,
}

// ユーザー情報更新

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserInfoRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserInfoResponse {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
}

// アカウント削除

#[derive(Debug, Deserialize, Validate)]
pub struct DeleteAccountRequest {
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteAccountResponse {
    pub success: bool,
    pub message: String,
}
