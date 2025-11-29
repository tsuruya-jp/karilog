use crate::dto::ammunition_limit_dto::{
    AmmunitionLimitResponse, CreateAmmunitionLimitRequest, UpdateAmmunitionLimitRequest,
};
use domain::entities::AmmunitionLimit;
use domain::repositories::AmmunitionLimitRepository;
use domain::value_objects::{AmmunitionLimitId, UserId};
use shared::error::AppError;
use std::sync::Arc;

/// 実包所持許可上限ユースケース
pub struct AmmunitionLimitUsecases {
    ammunition_limit_repository: Arc<dyn AmmunitionLimitRepository>,
}

impl AmmunitionLimitUsecases {
    pub fn new(ammunition_limit_repository: Arc<dyn AmmunitionLimitRepository>) -> Self {
        Self {
            ammunition_limit_repository,
        }
    }

    /// 実包所持許可上限を作成
    pub async fn create_ammunition_limit(
        &self,
        user_id: UserId,
        request: CreateAmmunitionLimitRequest,
    ) -> Result<AmmunitionLimitResponse, AppError> {
        // 同じ口径の上限が既に存在するか確認
        if self
            .ammunition_limit_repository
            .find_by_user_id_and_caliber(&user_id, &request.caliber)
            .await?
            .is_some()
        {
            return Err(AppError::ValidationError(format!(
                "Ammunition limit for caliber {} already exists",
                request.caliber
            )));
        }

        let ammunition_limit = AmmunitionLimit::new(user_id, request.caliber, request.max_quantity)
            .map_err(AppError::ValidationError)?;

        self.ammunition_limit_repository
            .save(&ammunition_limit)
            .await?;

        Ok(AmmunitionLimitResponse::from(ammunition_limit))
    }

    /// 実包所持許可上限を取得
    pub async fn get_ammunition_limit(
        &self,
        user_id: UserId,
        ammunition_limit_id: AmmunitionLimitId,
    ) -> Result<AmmunitionLimitResponse, AppError> {
        let ammunition_limit = self
            .ammunition_limit_repository
            .find_by_id(&ammunition_limit_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionLimit not found".to_string()))?;

        if ammunition_limit.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this ammunition limit".to_string(),
            ));
        }

        Ok(AmmunitionLimitResponse::from(ammunition_limit))
    }

    /// ユーザーの実包所持許可上限一覧を取得
    pub async fn list_ammunition_limits(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AmmunitionLimitResponse>, AppError> {
        let ammunition_limits = self
            .ammunition_limit_repository
            .find_by_user_id(&user_id)
            .await?;

        let responses = ammunition_limits
            .into_iter()
            .map(AmmunitionLimitResponse::from)
            .collect();

        Ok(responses)
    }

    /// 実包所持許可上限を更新
    pub async fn update_ammunition_limit(
        &self,
        user_id: UserId,
        ammunition_limit_id: AmmunitionLimitId,
        request: UpdateAmmunitionLimitRequest,
    ) -> Result<AmmunitionLimitResponse, AppError> {
        let mut ammunition_limit = self
            .ammunition_limit_repository
            .find_by_id(&ammunition_limit_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionLimit not found".to_string()))?;

        if ammunition_limit.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to update this ammunition limit".to_string(),
            ));
        }

        ammunition_limit
            .update_max_quantity(request.max_quantity)
            .map_err(AppError::ValidationError)?;

        self.ammunition_limit_repository
            .save(&ammunition_limit)
            .await?;

        Ok(AmmunitionLimitResponse::from(ammunition_limit))
    }

    /// 実包所持許可上限を削除
    pub async fn delete_ammunition_limit(
        &self,
        user_id: UserId,
        ammunition_limit_id: AmmunitionLimitId,
    ) -> Result<(), AppError> {
        let ammunition_limit = self
            .ammunition_limit_repository
            .find_by_id(&ammunition_limit_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionLimit not found".to_string()))?;

        if ammunition_limit.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this ammunition limit".to_string(),
            ));
        }

        self.ammunition_limit_repository
            .delete(&ammunition_limit_id)
            .await?;

        Ok(())
    }
}
