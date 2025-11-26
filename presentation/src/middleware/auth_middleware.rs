use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use domain::value_objects::UserId;
use infrastructure::services::JwtService;
use serde_json::json;
use std::sync::Arc;

/// 認証が必要なエンドポイントで使用するエクストラクタ
///
/// このエクストラクタを使用すると、Authorizationヘッダーから
/// JWTトークンを自動的に検証し、ユーザーIDを取得できます。
///
/// # Example
/// ```rust
/// async fn protected_endpoint(
///     AuthenticatedUser(user_id): AuthenticatedUser,
/// ) -> impl IntoResponse {
///     // user_idを使用した処理
/// }
/// ```
pub struct AuthenticatedUser(pub UserId);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ExtensionからJwtServiceを取得
        let Extension(jwt_service) = Extension::<Arc<JwtService>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| AuthError::InternalError)?;

        // Authorizationヘッダーを取得
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // "Bearer "プレフィックスを確認して除去
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        // トークンを検証してユーザーIDを取得
        let user_id = jwt_service
            .verify_access_token(token)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthenticatedUser(user_id))
    }
}

/// 認証エラー
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authorization token"),
            AuthError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

/// オプショナル認証エクストラクタ
///
/// 認証がオプションの場合に使用します。
/// トークンが有効な場合は Some(user_id) を、
/// トークンがないか無効な場合は None を返します。
pub struct OptionalAuthenticatedUser(pub Option<UserId>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ExtensionからJwtServiceを取得
        let jwt_service = match Extension::<Arc<JwtService>>::from_request_parts(parts, _state).await {
            Ok(Extension(service)) => service,
            Err(_) => return Ok(OptionalAuthenticatedUser(None)),
        };

        // Authorizationヘッダーを取得
        let auth_header = match parts.headers.get("Authorization").and_then(|h| h.to_str().ok()) {
            Some(header) => header,
            None => return Ok(OptionalAuthenticatedUser(None)),
        };

        // "Bearer "プレフィックスを確認して除去
        let token = match auth_header.strip_prefix("Bearer ") {
            Some(t) => t,
            None => return Ok(OptionalAuthenticatedUser(None)),
        };

        // トークンを検証してユーザーIDを取得
        let user_id = jwt_service.verify_access_token(token).ok();

        Ok(OptionalAuthenticatedUser(user_id))
    }
}
