use async_trait::async_trait;
use shared::error::AppError;
use tracing::info;

/// メール送信サービスのトレイト
#[async_trait]
pub trait EmailService: Send + Sync {
    /// メール確認用のメールを送信
    async fn send_verification_email(
        &self,
        to: &str,
        token: &str,
        verification_url: &str,
    ) -> Result<(), AppError>;

    /// パスワードリセット用のメールを送信
    async fn send_password_reset_email(
        &self,
        to: &str,
        token: &str,
        reset_url: &str,
    ) -> Result<(), AppError>;
}

/// コンソールログに出力するメール送信サービス（開発用）
#[derive(Clone)]
pub struct ConsoleEmailService;

impl ConsoleEmailService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for ConsoleEmailService {
    async fn send_verification_email(
        &self,
        to: &str,
        token: &str,
        verification_url: &str,
    ) -> Result<(), AppError> {
        info!("====================================");
        info!("メール確認メール送信（開発モード）");
        info!("宛先: {}", to);
        info!("トークン: {}", token);
        info!("確認URL: {}?token={}", verification_url, token);
        info!("====================================");
        Ok(())
    }

    async fn send_password_reset_email(
        &self,
        to: &str,
        token: &str,
        reset_url: &str,
    ) -> Result<(), AppError> {
        info!("====================================");
        info!("パスワードリセットメール送信（開発モード）");
        info!("宛先: {}", to);
        info!("トークン: {}", token);
        info!("リセットURL: {}?token={}", reset_url, token);
        info!("====================================");
        Ok(())
    }
}

/// SMTP経由でメールを送信するサービス（本番用・未実装）
#[derive(Clone)]
pub struct SmtpEmailService {
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from_address: String,
}

impl SmtpEmailService {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_address: String,
    ) -> Self {
        Self {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            from_address,
        }
    }
}

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_verification_email(
        &self,
        _to: &str,
        _token: &str,
        _verification_url: &str,
    ) -> Result<(), AppError> {
        // TODO: 実際のSMTP送信を実装
        Err(AppError::InternalServerError(
            "SMTP email service not implemented yet".to_string(),
        ))
    }

    async fn send_password_reset_email(
        &self,
        _to: &str,
        _token: &str,
        _reset_url: &str,
    ) -> Result<(), AppError> {
        // TODO: 実際のSMTP送信を実装
        Err(AppError::InternalServerError(
            "SMTP email service not implemented yet".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_console_email_service_verification() {
        let service = ConsoleEmailService::new();
        let result = service
            .send_verification_email(
                "test@example.com",
                "test_token",
                "http://localhost:3000/verify",
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_console_email_service_password_reset() {
        let service = ConsoleEmailService::new();
        let result = service
            .send_password_reset_email(
                "test@example.com",
                "test_token",
                "http://localhost:3000/reset-password",
            )
            .await;

        assert!(result.is_ok());
    }
}
