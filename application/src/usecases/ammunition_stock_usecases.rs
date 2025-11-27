use crate::dto::AmmunitionStockResponse;
use domain::repositories::{
    AmmunitionLimitRepository, AmmunitionPurchaseRepository, AmmunitionTypeRepository,
    AmmunitionUsageRepository,
};
use domain::services::AmmunitionStockService;
use domain::value_objects::{AmmunitionTypeId, UserId};
use shared::error::AppError;
use std::sync::Arc;

/// 実包在庫ユースケース
pub struct AmmunitionStockUsecases {
    ammunition_type_repository: Arc<dyn AmmunitionTypeRepository>,
    ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
    ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
    ammunition_limit_repository: Arc<dyn AmmunitionLimitRepository>,
    stock_service: AmmunitionStockService,
}

impl AmmunitionStockUsecases {
    pub fn new(
        ammunition_type_repository: Arc<dyn AmmunitionTypeRepository>,
        ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
        ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
        ammunition_limit_repository: Arc<dyn AmmunitionLimitRepository>,
    ) -> Self {
        let stock_service = AmmunitionStockService::new(
            ammunition_type_repository.clone(),
            ammunition_purchase_repository.clone(),
            ammunition_usage_repository.clone(),
            ammunition_limit_repository.clone(),
        );

        Self {
            ammunition_type_repository,
            ammunition_purchase_repository,
            ammunition_usage_repository,
            ammunition_limit_repository,
            stock_service,
        }
    }

    /// 在庫一覧を取得
    pub async fn list_stocks(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AmmunitionStockResponse>, AppError> {
        let stocks = self.stock_service.calculate_all_stocks(user_id).await?;

        Ok(stocks.into_iter().map(Into::into).collect())
    }

    /// 特定の実包種別の在庫を取得
    pub async fn get_stock(
        &self,
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
    ) -> Result<AmmunitionStockResponse, AppError> {
        // 実包種別が存在するか確認
        let ammunition_type = self
            .ammunition_type_repository
            .find_by_id(&ammunition_type_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Ammunition type not found".to_string()))?;

        // ユーザーの所有物か確認
        if ammunition_type.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this ammunition type".to_string(),
            ));
        }

        let stock = self
            .stock_service
            .calculate_stock(user_id, ammunition_type_id)
            .await?;

        Ok(stock.into())
    }
}
