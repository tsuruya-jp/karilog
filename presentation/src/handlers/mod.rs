// HTTPハンドラ
// APIエンドポイントのハンドラを定義

pub mod auth_handlers;

// Phase2: 銃砲・実包管理
pub mod ammunition_limit_handlers;
pub mod ammunition_purchase_handlers;
pub mod ammunition_stock_handlers;
pub mod ammunition_type_handlers;
pub mod ammunition_usage_handlers;
pub mod firearm_handlers;

// Phase3: 出猟管理
pub mod hunting_record_handlers;

pub use auth_handlers::*;

// Phase2
pub use ammunition_limit_handlers::*;
pub use ammunition_purchase_handlers::*;
pub use ammunition_stock_handlers::*;
pub use ammunition_type_handlers::*;
pub use ammunition_usage_handlers::*;
pub use firearm_handlers::*;

// Phase3
pub use hunting_record_handlers::*;
