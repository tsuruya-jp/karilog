use crate::dto::{
    DeleteAccountRequest, DeleteAccountResponse, LoginRequest, LoginResponse, LogoutRequest,
    LogoutResponse, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
    RegisterResponse, RequestPasswordResetRequest, RequestPasswordResetResponse,
    ResendVerificationEmailRequest, ResendVerificationEmailResponse, ResetPasswordRequest,
    ResetPasswordResponse, UpdateUserInfoRequest, UpdateUserInfoResponse, UserInfoResponse,
    VerifyEmailRequest, VerifyEmailResponse,
};
use domain::entities::{RefreshToken, User};
use domain::repositories::UserRepository;
use domain::value_objects::{Email, Password, UserId};
use infrastructure::services::{EmailService, JwtService, PasswordService};
use shared::error::AppError;
use std::sync::Arc;
use uuid::Uuid;

/// 認証ユースケース
#[derive(Clone)]
pub struct AuthUseCases<R: UserRepository, E: EmailService> {
    user_repository: Arc<R>,
    jwt_service: Arc<JwtService>,
    email_service: Arc<E>,
    password_service: Arc<PasswordService>,
    base_url: String,
}

impl<R: UserRepository, E: EmailService> AuthUseCases<R, E> {
    pub fn new(
        user_repository: Arc<R>,
        jwt_service: Arc<JwtService>,
        email_service: Arc<E>,
        password_service: Arc<PasswordService>,
        base_url: String,
    ) -> Self {
        Self {
            user_repository,
            jwt_service,
            email_service,
            password_service,
            base_url,
        }
    }

    /// ユーザー登録
    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, AppError> {
        // メールアドレスのバリデーション
        let email = Email::new(request.email)
            .map_err(|e| AppError::ValidationError(format!("Invalid email: {:?}", e)))?;

        // パスワードのバリデーション
        let password = Password::new(request.password)
            .map_err(|e| AppError::ValidationError(format!("Invalid password: {:?}", e)))?;

        // メールアドレスの重複チェック
        if let Some(_) = self.user_repository.find_by_email(&email).await? {
            return Err(AppError::BadRequest(
                "Email address already registered".to_string(),
            ));
        }

        // パスワードをハッシュ化
        let password_hash = self.password_service.hash_password(&password)?;

        // ユーザーエンティティを作成
        let mut user = User::new(email.clone(), password_hash, request.name);

        // メール確認トークンを生成して設定
        let verification_token = Uuid::new_v4().to_string();
        user.set_email_verification_token(verification_token.clone(), 24); // 24時間有効

        // ユーザーを保存
        self.user_repository.save(&user).await?;

        // 確認メールを送信
        let verification_url = format!("{}/api/v1/auth/verify-email", self.base_url);
        self.email_service
            .send_verification_email(email.as_str(), &verification_token, &verification_url)
            .await?;

        // アクセストークンとリフレッシュトークンを生成
        let access_token = self.jwt_service.generate_access_token(&user.id)?;
        let refresh_token_str = self.jwt_service.generate_refresh_token(&user.id)?;

        // リフレッシュトークンをハッシュ化して保存
        let refresh_token_hash = format!("{:x}", md5::compute(refresh_token_str.as_bytes()));
        let refresh_token = RefreshToken::new(
            user.id,
            refresh_token_hash,
            self.jwt_service.get_refresh_token_expires_in_days(),
        );
        self.user_repository.save_refresh_token(&refresh_token).await?;

        Ok(RegisterResponse {
            user_id: user.id.to_string(),
            email: email.as_str().to_string(),
            access_token,
            refresh_token: refresh_token_str,
        })
    }

    /// ログイン
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AppError> {
        // メールアドレスのバリデーション
        let email = Email::new(request.email)
            .map_err(|e| AppError::ValidationError(format!("Invalid email: {:?}", e)))?;

        // ユーザーを検索
        let mut user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // アカウントロックチェック
        if user.is_locked() {
            return Err(AppError::Unauthorized(
                "Account is locked. Please try again later.".to_string(),
            ));
        }

        // パスワード検証
        let is_valid = self
            .password_service
            .verify_password(&request.password, &user.password_hash)?;

        if !is_valid {
            // ログイン失敗をカウント
            user.increment_failed_login_attempts();
            self.user_repository.save(&user).await?;

            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // ログイン成功：失敗カウントをリセット
        user.reset_failed_login_attempts();
        self.user_repository.save(&user).await?;

        // アクセストークンとリフレッシュトークンを生成
        let access_token = self.jwt_service.generate_access_token(&user.id)?;
        let refresh_token_str = self.jwt_service.generate_refresh_token(&user.id)?;

        // リフレッシュトークンをハッシュ化して保存
        let refresh_token_hash = format!("{:x}", md5::compute(refresh_token_str.as_bytes()));
        let refresh_token = RefreshToken::new(
            user.id,
            refresh_token_hash,
            self.jwt_service.get_refresh_token_expires_in_days(),
        );
        self.user_repository.save_refresh_token(&refresh_token).await?;

        Ok(LoginResponse {
            user_id: user.id.to_string(),
            email: email.as_str().to_string(),
            access_token,
            refresh_token: refresh_token_str,
        })
    }

    /// ログアウト
    pub async fn logout(&self, request: LogoutRequest) -> Result<LogoutResponse, AppError> {
        // リフレッシュトークンをハッシュ化
        let token_hash = format!("{:x}", md5::compute(request.refresh_token.as_bytes()));

        // リフレッシュトークンを検索
        if let Some(mut token) = self
            .user_repository
            .find_refresh_token_by_token_hash(&token_hash)
            .await?
        {
            // トークンを無効化
            token.revoke();
            self.user_repository.save_refresh_token(&token).await?;
        }

        Ok(LogoutResponse {
            success: true,
            message: "Logged out successfully".to_string(),
        })
    }

    /// トークンリフレッシュ
    pub async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AppError> {
        // リフレッシュトークンを検証
        let user_id = self
            .jwt_service
            .verify_refresh_token(&request.refresh_token)?;

        // リフレッシュトークンをハッシュ化
        let token_hash = format!("{:x}", md5::compute(request.refresh_token.as_bytes()));

        // リフレッシュトークンを検索
        let stored_token = self
            .user_repository
            .find_refresh_token_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        // トークンの有効性をチェック
        if !stored_token.is_valid() {
            return Err(AppError::Unauthorized(
                "Refresh token has been revoked or expired".to_string(),
            ));
        }

        // ユーザーIDの一致を確認
        if stored_token.user_id != user_id {
            return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
        }

        // 新しいアクセストークンとリフレッシュトークンを生成
        let new_access_token = self.jwt_service.generate_access_token(&user_id)?;
        let new_refresh_token_str = self.jwt_service.generate_refresh_token(&user_id)?;

        // 古いリフレッシュトークンを削除
        self.user_repository
            .delete_refresh_token(&stored_token.id)
            .await?;

        // 新しいリフレッシュトークンをハッシュ化して保存
        let new_refresh_token_hash =
            format!("{:x}", md5::compute(new_refresh_token_str.as_bytes()));
        let new_refresh_token = RefreshToken::new(
            user_id,
            new_refresh_token_hash,
            self.jwt_service.get_refresh_token_expires_in_days(),
        );
        self.user_repository
            .save_refresh_token(&new_refresh_token)
            .await?;

        Ok(RefreshTokenResponse {
            access_token: new_access_token,
            refresh_token: new_refresh_token_str,
        })
    }

    /// メール確認
    pub async fn verify_email(
        &self,
        request: VerifyEmailRequest,
    ) -> Result<VerifyEmailResponse, AppError> {
        // トークンでユーザーを検索
        let mut user = self
            .user_repository
            .find_by_email_verification_token(&request.token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid verification token".to_string()))?;

        // トークンの有効性を確認
        if !user.is_email_verification_token_valid(&request.token) {
            return Ok(VerifyEmailResponse {
                success: false,
                message: "Verification token has expired or is invalid".to_string(),
            });
        }

        // メール確認を完了
        user.verify_email();
        self.user_repository.save(&user).await?;

        Ok(VerifyEmailResponse {
            success: true,
            message: "Email verified successfully".to_string(),
        })
    }

    /// パスワードリセットリクエスト
    pub async fn request_password_reset(
        &self,
        request: RequestPasswordResetRequest,
    ) -> Result<RequestPasswordResetResponse, AppError> {
        // メールアドレスのバリデーション
        let email = Email::new(request.email)
            .map_err(|e| AppError::ValidationError(format!("Invalid email: {:?}", e)))?;

        // ユーザーを検索
        if let Some(mut user) = self.user_repository.find_by_email(&email).await? {
            // パスワードリセットトークンを生成して設定
            let reset_token = Uuid::new_v4().to_string();
            user.set_password_reset_token(reset_token.clone(), 1); // 1時間有効

            // ユーザーを保存
            self.user_repository.save(&user).await?;

            // パスワードリセットメールを送信
            let reset_url = format!("{}/api/v1/auth/password/reset", self.base_url);
            self.email_service
                .send_password_reset_email(email.as_str(), &reset_token, &reset_url)
                .await?;
        }

        // セキュリティ上、ユーザーが存在しない場合でも同じレスポンスを返す
        Ok(RequestPasswordResetResponse {
            success: true,
            message: "If the email exists, a password reset link has been sent".to_string(),
        })
    }

    /// パスワードリセット
    pub async fn reset_password(
        &self,
        request: ResetPasswordRequest,
    ) -> Result<ResetPasswordResponse, AppError> {
        // パスワードのバリデーション
        let new_password = Password::new(request.new_password)
            .map_err(|e| AppError::ValidationError(format!("Invalid password: {:?}", e)))?;

        // トークンでユーザーを検索
        let mut user = self
            .user_repository
            .find_by_password_reset_token(&request.token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid reset token".to_string()))?;

        // トークンの有効性を確認
        if !user.is_password_reset_token_valid(&request.token) {
            return Ok(ResetPasswordResponse {
                success: false,
                message: "Reset token has expired or is invalid".to_string(),
            });
        }

        // パスワードをハッシュ化
        let new_password_hash = self.password_service.hash_password(&new_password)?;

        // パスワードをリセット
        user.reset_password(new_password_hash);
        self.user_repository.save(&user).await?;

        // すべてのリフレッシュトークンを無効化（セキュリティ対策）
        self.user_repository
            .revoke_all_refresh_tokens_for_user(&user.id)
            .await?;

        Ok(ResetPasswordResponse {
            success: true,
            message: "Password reset successfully".to_string(),
        })
    }

    /// ユーザー情報取得
    pub async fn get_user_info(&self, user_id: &UserId) -> Result<UserInfoResponse, AppError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserInfoResponse {
            user_id: user.id.to_string(),
            email: user.email.as_str().to_string(),
            name: user.name.clone(),
            email_verified: user.email_verified,
        })
    }

    /// 確認メール再送信
    pub async fn resend_verification_email(
        &self,
        request: ResendVerificationEmailRequest,
    ) -> Result<ResendVerificationEmailResponse, AppError> {
        // メールアドレスのバリデーション
        let email = Email::new(request.email)
            .map_err(|e| AppError::ValidationError(format!("Invalid email: {:?}", e)))?;

        // ユーザーを検索
        if let Some(mut user) = self.user_repository.find_by_email(&email).await? {
            // 既にメール確認済みの場合
            if user.email_verified {
                return Ok(ResendVerificationEmailResponse {
                    success: true,
                    message: "Email already verified".to_string(),
                });
            }

            // 新しい確認トークンを生成
            let verification_token = Uuid::new_v4().to_string();
            user.set_email_verification_token(verification_token.clone(), 24); // 24時間有効

            // ユーザーを保存
            self.user_repository.save(&user).await?;

            // 確認メールを送信
            let verification_url = format!("{}/api/v1/auth/verify-email", self.base_url);
            self.email_service
                .send_verification_email(email.as_str(), &verification_token, &verification_url)
                .await?;
        }

        // セキュリティ上、ユーザーが存在しない場合でも同じレスポンスを返す
        Ok(ResendVerificationEmailResponse {
            success: true,
            message: "If the email exists, a verification link has been sent".to_string(),
        })
    }

    /// ユーザー情報更新
    pub async fn update_user_info(
        &self,
        user_id: &UserId,
        request: UpdateUserInfoRequest,
    ) -> Result<UpdateUserInfoResponse, AppError> {
        // ユーザーを検索
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // 名前を更新
        user.update_name(request.name);

        // ユーザーを保存
        self.user_repository.save(&user).await?;

        Ok(UpdateUserInfoResponse {
            user_id: user.id.to_string(),
            email: user.email.as_str().to_string(),
            name: user.name.clone(),
            email_verified: user.email_verified,
        })
    }

    /// アカウント削除
    pub async fn delete_account(
        &self,
        user_id: &UserId,
        request: DeleteAccountRequest,
    ) -> Result<DeleteAccountResponse, AppError> {
        // ユーザーを検索
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // パスワード検証
        let is_valid = self
            .password_service
            .verify_password(&request.password, &user.password_hash)?;

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid password".to_string()));
        }

        // アカウントを削除（論理削除）
        user.delete();
        self.user_repository.save(&user).await?;

        // すべてのリフレッシュトークンを無効化
        self.user_repository
            .revoke_all_refresh_tokens_for_user(&user.id)
            .await?;

        Ok(DeleteAccountResponse {
            success: true,
            message: "Account deleted successfully".to_string(),
        })
    }
}
