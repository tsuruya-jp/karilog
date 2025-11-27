use crate::dto::ammunition_usage_dto::{
    AmmunitionUsageResponse, CreateAmmunitionUsageRequest, UpdateAmmunitionUsageRequest,
};
use domain::entities::AmmunitionUsage;
use domain::repositories::AmmunitionUsageRepository;
use domain::value_objects::{AmmunitionTypeId, AmmunitionUsageId, FirearmId, UserId};
use shared::error::AppError;
use std::sync::Arc;
use uuid::Uuid;

/// 実包使用記録ユースケース
pub struct AmmunitionUsageUsecases {
    ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>,
}

impl AmmunitionUsageUsecases {
    pub fn new(ammunition_usage_repository: Arc<dyn AmmunitionUsageRepository>) -> Self {
        Self {
            ammunition_usage_repository,
        }
    }

    /// 実包使用記録を作成
    pub async fn create_ammunition_usage(
        &self,
        user_id: UserId,
        request: CreateAmmunitionUsageRequest,
    ) -> Result<AmmunitionUsageResponse, AppError> {
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

        let hunting_record_id = request
            .hunting_record_id
            .as_ref()
            .map(|id| {
                Uuid::parse_str(id)
                    .map_err(|e| AppError::ValidationError(format!("Invalid hunting_record_id: {}", e)))
            })
            .transpose()?;

        let ammunition_usage = AmmunitionUsage::new(
            user_id,
            ammunition_type_id,
            firearm_id,
            hunting_record_id,
            request.usage_date,
            request.location,
            request.quantity_used,
            request.notes,
        )
        .map_err(|e| AppError::ValidationError(e))?;

        self.ammunition_usage_repository
            .save(&ammunition_usage)
            .await?;

        Ok(AmmunitionUsageResponse::from(ammunition_usage))
    }

    /// 実包使用記録を取得
    pub async fn get_ammunition_usage(
        &self,
        user_id: UserId,
        ammunition_usage_id: AmmunitionUsageId,
    ) -> Result<AmmunitionUsageResponse, AppError> {
        let ammunition_usage = self
            .ammunition_usage_repository
            .find_by_id(&ammunition_usage_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionUsage not found".to_string()))?;

        if ammunition_usage.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this ammunition usage".to_string(),
            ));
        }

        Ok(AmmunitionUsageResponse::from(ammunition_usage))
    }

    /// ユーザーの実包使用記録一覧を取得
    pub async fn list_ammunition_usages(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AmmunitionUsageResponse>, AppError> {
        let ammunition_usages = self
            .ammunition_usage_repository
            .find_by_user_id(&user_id)
            .await?;

        let responses = ammunition_usages
            .into_iter()
            .map(AmmunitionUsageResponse::from)
            .collect();

        Ok(responses)
    }

    /// 実包使用記録を更新
    pub async fn update_ammunition_usage(
        &self,
        user_id: UserId,
        ammunition_usage_id: AmmunitionUsageId,
        request: UpdateAmmunitionUsageRequest,
    ) -> Result<AmmunitionUsageResponse, AppError> {
        let mut ammunition_usage = self
            .ammunition_usage_repository
            .find_by_id(&ammunition_usage_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionUsage not found".to_string()))?;

        if ammunition_usage.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to update this ammunition usage".to_string(),
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

        let hunting_record_id = request
            .hunting_record_id
            .as_ref()
            .map(|id| {
                Uuid::parse_str(id)
                    .map_err(|e| AppError::ValidationError(format!("Invalid hunting_record_id: {}", e)))
            })
            .transpose()?;

        ammunition_usage
            .update(
                ammunition_type_id,
                firearm_id,
                hunting_record_id,
                request.usage_date,
                request.location,
                request.quantity_used,
                request.notes,
            )
            .map_err(|e| AppError::ValidationError(e))?;

        self.ammunition_usage_repository
            .save(&ammunition_usage)
            .await?;

        Ok(AmmunitionUsageResponse::from(ammunition_usage))
    }

    /// 実包使用記録を削除（論理削除）
    pub async fn delete_ammunition_usage(
        &self,
        user_id: UserId,
        ammunition_usage_id: AmmunitionUsageId,
    ) -> Result<(), AppError> {
        let ammunition_usage = self
            .ammunition_usage_repository
            .find_by_id(&ammunition_usage_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionUsage not found".to_string()))?;

        if ammunition_usage.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this ammunition usage".to_string(),
            ));
        }

        self.ammunition_usage_repository
            .delete(&ammunition_usage_id)
            .await?;

        Ok(())
    }
}
