// Data Transfer Objects
// リクエスト/レスポンスのDTOを定義

pub mod auth_dto;

// Phase2: 銃砲・実包管理
pub mod ammunition_limit_dto;
pub mod ammunition_purchase_dto;
pub mod ammunition_stock_dto;
pub mod ammunition_type_dto;
pub mod ammunition_usage_dto;
pub mod firearm_dto;

// Phase3: 出猟管理
pub mod hunting_record_dto;

// Phase4: 帳簿出力
pub mod report_dto;

pub use auth_dto::*;

// Phase2
pub use ammunition_limit_dto::*;
pub use ammunition_purchase_dto::*;
pub use ammunition_stock_dto::*;
pub use ammunition_type_dto::*;
pub use ammunition_usage_dto::*;
pub use firearm_dto::*;

// Phase3
pub use hunting_record_dto::*;

// Phase4
pub use report_dto::*;
