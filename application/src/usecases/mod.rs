// ユースケース
// アプリケーションのユースケースを定義

pub mod auth_usecases;

// Phase2: 銃砲・実包管理
pub mod ammunition_limit_usecases;
pub mod ammunition_purchase_usecases;
pub mod ammunition_stock_usecases;
pub mod ammunition_type_usecases;
pub mod ammunition_usage_usecases;
pub mod firearm_usecases;

// Phase3: 出猟管理
pub mod hunting_record_usecases;

pub use auth_usecases::AuthUseCases;

// Phase2
pub use ammunition_limit_usecases::AmmunitionLimitUsecases;
pub use ammunition_purchase_usecases::AmmunitionPurchaseUsecases;
pub use ammunition_stock_usecases::AmmunitionStockUsecases;
pub use ammunition_type_usecases::AmmunitionTypeUsecases;
pub use ammunition_usage_usecases::AmmunitionUsageUsecases;
pub use firearm_usecases::FirearmUsecases;

// Phase3
pub use hunting_record_usecases::HuntingRecordUsecases;
