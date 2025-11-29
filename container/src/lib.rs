// DIコンテナ
// 依存性注入とライフサイクル管理を定義

use anyhow::Result;
use application::usecases::{
    AmmunitionLimitUsecases, AmmunitionPurchaseUsecases, AmmunitionStockUsecases,
    AmmunitionTypeUsecases, AmmunitionUsageUsecases, AuthUseCases, FirearmUsecases,
    HuntingRecordUsecases, ReportUsecases,
};
use infrastructure::persistence::{
    PostgresAmmunitionLimitRepository, PostgresAmmunitionPurchaseRepository,
    PostgresAmmunitionTypeRepository, PostgresAmmunitionUsageRepository, PostgresFirearmRepository,
    PostgresHuntingRecordRepository, PostgresUserRepository,
};
use infrastructure::services::{
    ConsoleEmailService, CsvService, JwtService, PasswordService, PdfService,
};
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
    // Phase2: 銃砲・実包管理
    pub firearm_repository: Arc<PostgresFirearmRepository>,
    pub ammunition_type_repository: Arc<PostgresAmmunitionTypeRepository>,
    pub ammunition_limit_repository: Arc<PostgresAmmunitionLimitRepository>,
    pub ammunition_purchase_repository: Arc<PostgresAmmunitionPurchaseRepository>,
    pub ammunition_usage_repository: Arc<PostgresAmmunitionUsageRepository>,
    pub firearm_usecases: Arc<FirearmUsecases>,
    pub ammunition_type_usecases: Arc<AmmunitionTypeUsecases>,
    pub ammunition_limit_usecases: Arc<AmmunitionLimitUsecases>,
    pub ammunition_purchase_usecases: Arc<AmmunitionPurchaseUsecases>,
    pub ammunition_usage_usecases: Arc<AmmunitionUsageUsecases>,
    pub ammunition_stock_usecases: Arc<AmmunitionStockUsecases>,
    // Phase3: 出猟管理
    pub hunting_record_repository: Arc<PostgresHuntingRecordRepository>,
    pub hunting_record_usecases: Arc<HuntingRecordUsecases>,
    // Phase4: 帳簿出力
    pub pdf_service: Arc<PdfService>,
    pub csv_service: Arc<CsvService>,
    pub report_usecases: Arc<ReportUsecases>,
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

        // Phase2: 銃砲・実包管理リポジトリ
        let firearm_repository = Arc::new(PostgresFirearmRepository::new(db_pool.clone()));
        let ammunition_type_repository =
            Arc::new(PostgresAmmunitionTypeRepository::new(db_pool.clone()));
        let ammunition_limit_repository =
            Arc::new(PostgresAmmunitionLimitRepository::new(db_pool.clone()));
        let ammunition_purchase_repository =
            Arc::new(PostgresAmmunitionPurchaseRepository::new(db_pool.clone()));
        let ammunition_usage_repository =
            Arc::new(PostgresAmmunitionUsageRepository::new(db_pool.clone()));

        // Phase3: 出猟管理リポジトリ
        let hunting_record_repository =
            Arc::new(PostgresHuntingRecordRepository::new(db_pool.clone()));

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

        // Phase2: 銃砲・実包管理ユースケース
        let firearm_usecases = Arc::new(FirearmUsecases::new(firearm_repository.clone()));
        let ammunition_type_usecases = Arc::new(AmmunitionTypeUsecases::new(
            ammunition_type_repository.clone(),
        ));
        let ammunition_limit_usecases = Arc::new(AmmunitionLimitUsecases::new(
            ammunition_limit_repository.clone(),
        ));
        let ammunition_purchase_usecases = Arc::new(AmmunitionPurchaseUsecases::new(
            ammunition_purchase_repository.clone(),
        ));
        let ammunition_usage_usecases = Arc::new(AmmunitionUsageUsecases::new(
            ammunition_usage_repository.clone(),
        ));
        let ammunition_stock_usecases = Arc::new(AmmunitionStockUsecases::new(
            ammunition_type_repository.clone(),
            ammunition_purchase_repository.clone(),
            ammunition_usage_repository.clone(),
            ammunition_limit_repository.clone(),
        ));

        // Phase3: 出猟管理ユースケース
        let hunting_record_usecases = Arc::new(HuntingRecordUsecases::new(
            hunting_record_repository.clone(),
        ));

        // Phase4: 帳簿出力サービスとユースケース
        let pdf_service = Arc::new(PdfService::new());
        let csv_service = Arc::new(CsvService::new());
        let report_usecases = Arc::new(ReportUsecases::new(
            ammunition_purchase_repository.clone(),
            ammunition_usage_repository.clone(),
            hunting_record_repository.clone(),
            pdf_service.clone(),
            csv_service.clone(),
        ));

        Ok(Arc::new(Self {
            config,
            db_pool,
            user_repository,
            jwt_service,
            email_service,
            password_service,
            auth_usecases,
            firearm_repository,
            ammunition_type_repository,
            ammunition_limit_repository,
            ammunition_purchase_repository,
            ammunition_usage_repository,
            firearm_usecases,
            ammunition_type_usecases,
            ammunition_limit_usecases,
            ammunition_purchase_usecases,
            ammunition_usage_usecases,
            ammunition_stock_usecases,
            hunting_record_repository,
            hunting_record_usecases,
            pdf_service,
            csv_service,
            report_usecases,
        }))
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
