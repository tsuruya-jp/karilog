// ドメインサービス
// 複数のエンティティにまたがるビジネスロジックを定義

// Phase2: 銃砲・実包管理
pub mod ammunition_stock_service;

pub use ammunition_stock_service::{AmmunitionStock, AmmunitionStockService};
