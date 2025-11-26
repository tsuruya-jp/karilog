// DIコンテナ
// 依存性注入とライフサイクル管理を定義

use anyhow::Result;
use application::usecases::AuthUseCases;
use infrastructure::persistence::PostgresUserRepository;
use infrastructure::services::{ConsoleEmailService, JwtService, PasswordService};
use shared::config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub struct Container {
    pub config: AppConfig,
    pub db_pool: Pool<Postgres>,
    pub user_repository: Arc<PostgresUserRepository>,
    pub jwt_service: Arc<JwtService>,
    pub email_service: Arc<ConsoleEmailService>,
    pub password_service: Arc<PasswordService>,
    pub auth_usecases: Arc<AuthUseCases<PostgresUserRepository, ConsoleEmailService>>,
}

impl Container {
    pub async fn new(config: AppConfig) -> Result<Arc<Self>> {
        // データベース接続プールを作成
        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.url)
            .await?;

        // リポジトリを作成
        let user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));

        // サービスを作成
        let jwt_service = Arc::new(JwtService::new(
            config.jwt.secret.clone(),
            config.jwt.access_token_expires_in_minutes,
            config.jwt.refresh_token_expires_in_days,
        ));

        let email_service = Arc::new(ConsoleEmailService::new());
        let password_service = Arc::new(PasswordService::new());

        // ユースケースを作成
        let auth_usecases = Arc::new(AuthUseCases::new(
            user_repository.clone(),
            jwt_service.clone(),
            email_service.clone(),
            password_service.clone(),
            config.server.base_url.clone(),
        ));

        Ok(Arc::new(Self {
            config,
            db_pool,
            user_repository,
            jwt_service,
            email_service,
            password_service,
            auth_usecases,
        }))
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
