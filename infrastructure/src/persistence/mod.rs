// データ永続化実装
// リポジトリの実装を定義

pub mod postgres_user_repository;

// Phase2: 銃砲・実包管理
pub mod postgres_ammunition_limit_repository;
pub mod postgres_ammunition_purchase_repository;
pub mod postgres_ammunition_type_repository;
pub mod postgres_ammunition_usage_repository;
pub mod postgres_firearm_repository;

// Phase3: 出猟管理
pub mod postgres_hunting_record_repository;

pub use postgres_user_repository::PostgresUserRepository;

// Phase2
pub use postgres_ammunition_limit_repository::PostgresAmmunitionLimitRepository;
pub use postgres_ammunition_purchase_repository::PostgresAmmunitionPurchaseRepository;
pub use postgres_ammunition_type_repository::PostgresAmmunitionTypeRepository;
pub use postgres_ammunition_usage_repository::PostgresAmmunitionUsageRepository;
pub use postgres_firearm_repository::PostgresFirearmRepository;

// Phase3
pub use postgres_hunting_record_repository::PostgresHuntingRecordRepository;
