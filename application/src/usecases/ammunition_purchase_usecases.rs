use crate::dto::ammunition_purchase_dto::{
    AmmunitionPurchaseResponse, CreateAmmunitionPurchaseRequest, UpdateAmmunitionPurchaseRequest,
};
use domain::entities::AmmunitionPurchase;
use domain::repositories::AmmunitionPurchaseRepository;
use domain::value_objects::{AmmunitionPurchaseId, AmmunitionTypeId, FirearmId, UserId};
use shared::error::AppError;
use std::sync::Arc;

/// 実包購入記録ユースケース
pub struct AmmunitionPurchaseUsecases {
    ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>,
}

impl AmmunitionPurchaseUsecases {
    pub fn new(ammunition_purchase_repository: Arc<dyn AmmunitionPurchaseRepository>) -> Self {
        Self {
            ammunition_purchase_repository,
        }
    }

    /// 実包購入記録を作成
    pub async fn create_ammunition_purchase(
        &self,
        user_id: UserId,
        request: CreateAmmunitionPurchaseRequest,
    ) -> Result<AmmunitionPurchaseResponse, AppError> {
        let ammunition_type_id = AmmunitionTypeId::from_str(&request.ammunition_type_id)
            .map_err(|e| AppError::ValidationError(format!("Invalid ammunition_type_id: {}", e)))?;

        let firearm_id = request
            .firearm_id
            .as_ref()
            .map(|id| {
                FirearmId::from_str(id)
                    .map_err(|e| AppError::ValidationError(format!("Invalid firearm_id: {}", e)))
            })
            .transpose()?;

        let ammunition_purchase = AmmunitionPurchase::new(
            user_id,
            ammunition_type_id,
            firearm_id,
            request.purchase_date,
            request.supplier,
            request.quantity,
            request.price,
            request.notes,
        )
        .map_err(AppError::ValidationError)?;

        self.ammunition_purchase_repository
            .save(&ammunition_purchase)
            .await?;

        Ok(AmmunitionPurchaseResponse::from(ammunition_purchase))
    }

    /// 実包購入記録を取得
    pub async fn get_ammunition_purchase(
        &self,
        user_id: UserId,
        ammunition_purchase_id: AmmunitionPurchaseId,
    ) -> Result<AmmunitionPurchaseResponse, AppError> {
        let ammunition_purchase = self
            .ammunition_purchase_repository
            .find_by_id(&ammunition_purchase_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionPurchase not found".to_string()))?;

        if ammunition_purchase.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this ammunition purchase".to_string(),
            ));
        }

        Ok(AmmunitionPurchaseResponse::from(ammunition_purchase))
    }

    /// ユーザーの実包購入記録一覧を取得
    pub async fn list_ammunition_purchases(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AmmunitionPurchaseResponse>, AppError> {
        let ammunition_purchases = self
            .ammunition_purchase_repository
            .find_by_user_id(&user_id)
            .await?;

        let responses = ammunition_purchases
            .into_iter()
            .map(AmmunitionPurchaseResponse::from)
            .collect();

        Ok(responses)
    }

    /// 実包購入記録を更新
    pub async fn update_ammunition_purchase(
        &self,
        user_id: UserId,
        ammunition_purchase_id: AmmunitionPurchaseId,
        request: UpdateAmmunitionPurchaseRequest,
    ) -> Result<AmmunitionPurchaseResponse, AppError> {
        let mut ammunition_purchase = self
            .ammunition_purchase_repository
            .find_by_id(&ammunition_purchase_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionPurchase not found".to_string()))?;

        if ammunition_purchase.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to update this ammunition purchase".to_string(),
            ));
        }

        let ammunition_type_id = AmmunitionTypeId::from_str(&request.ammunition_type_id)
            .map_err(|e| AppError::ValidationError(format!("Invalid ammunition_type_id: {}", e)))?;

        let firearm_id = request
            .firearm_id
            .as_ref()
            .map(|id| {
                FirearmId::from_str(id)
                    .map_err(|e| AppError::ValidationError(format!("Invalid firearm_id: {}", e)))
            })
            .transpose()?;

        ammunition_purchase
            .update(
                ammunition_type_id,
                firearm_id,
                request.purchase_date,
                request.supplier,
                request.quantity,
                request.price,
                request.notes,
            )
            .map_err(AppError::ValidationError)?;

        self.ammunition_purchase_repository
            .save(&ammunition_purchase)
            .await?;

        Ok(AmmunitionPurchaseResponse::from(ammunition_purchase))
    }

    /// 実包購入記録を削除（論理削除）
    pub async fn delete_ammunition_purchase(
        &self,
        user_id: UserId,
        ammunition_purchase_id: AmmunitionPurchaseId,
    ) -> Result<(), AppError> {
        let ammunition_purchase = self
            .ammunition_purchase_repository
            .find_by_id(&ammunition_purchase_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionPurchase not found".to_string()))?;

        if ammunition_purchase.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this ammunition purchase".to_string(),
            ));
        }

        self.ammunition_purchase_repository
            .delete(&ammunition_purchase_id)
            .await?;

        Ok(())
    }
}
