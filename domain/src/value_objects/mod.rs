// 値オブジェクト
// メールアドレス、パスワードなどの値オブジェクトを定義

pub mod email;
pub mod password;
pub mod user_id;

// Phase2: 銃砲・実包管理
pub mod ammunition_limit_id;
pub mod ammunition_purchase_id;
pub mod ammunition_type_id;
pub mod ammunition_usage_id;
pub mod firearm_id;

// Phase3: 出猟管理
pub mod hunting_record_id;

pub use email::{Email, EmailError};
pub use password::{Password, PasswordError, PasswordHash};
pub use user_id::UserId;

// Phase2
pub use ammunition_limit_id::AmmunitionLimitId;
pub use ammunition_purchase_id::AmmunitionPurchaseId;
pub use ammunition_type_id::AmmunitionTypeId;
pub use ammunition_usage_id::AmmunitionUsageId;
pub use firearm_id::FirearmId;

// Phase3
pub use hunting_record_id::HuntingRecordId;
