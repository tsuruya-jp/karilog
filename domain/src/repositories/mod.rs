// リポジトリインターフェース
// データ永続化の抽象インターフェースを定義

pub mod user_repository;

// Phase2: 銃砲・実包管理
pub mod ammunition_limit_repository;
pub mod ammunition_purchase_repository;
pub mod ammunition_type_repository;
pub mod ammunition_usage_repository;
pub mod firearm_repository;

// Phase3: 出猟管理
pub mod hunting_record_repository;

pub use user_repository::UserRepository;

// Phase2
pub use ammunition_limit_repository::AmmunitionLimitRepository;
pub use ammunition_purchase_repository::AmmunitionPurchaseRepository;
pub use ammunition_type_repository::AmmunitionTypeRepository;
pub use ammunition_usage_repository::AmmunitionUsageRepository;
pub use firearm_repository::FirearmRepository;

// Phase3
pub use hunting_record_repository::HuntingRecordRepository;
