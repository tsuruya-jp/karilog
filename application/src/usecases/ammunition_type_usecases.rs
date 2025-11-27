use crate::dto::ammunition_type_dto::{
    AmmunitionTypeResponse, CreateAmmunitionTypeRequest, UpdateAmmunitionTypeRequest,
};
use domain::entities::AmmunitionType;
use domain::repositories::AmmunitionTypeRepository;
use domain::value_objects::{AmmunitionTypeId, UserId};
use shared::error::AppError;
use std::sync::Arc;

/// 実包種別ユースケース
pub struct AmmunitionTypeUsecases {
    ammunition_type_repository: Arc<dyn AmmunitionTypeRepository>,
}

impl AmmunitionTypeUsecases {
    pub fn new(ammunition_type_repository: Arc<dyn AmmunitionTypeRepository>) -> Self {
        Self {
            ammunition_type_repository,
        }
    }

    /// 実包種別を作成
    pub async fn create_ammunition_type(
        &self,
        user_id: UserId,
        request: CreateAmmunitionTypeRequest,
    ) -> Result<AmmunitionTypeResponse, AppError> {
        let ammunition_type = AmmunitionType::new(
            user_id,
            request.name,
            request.caliber,
            request.shot_size,
            request.is_slug,
        );

        self.ammunition_type_repository
            .save(&ammunition_type)
            .await?;

        Ok(AmmunitionTypeResponse::from(ammunition_type))
    }

    /// 実包種別を取得
    pub async fn get_ammunition_type(
        &self,
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
    ) -> Result<AmmunitionTypeResponse, AppError> {
        let ammunition_type = self
            .ammunition_type_repository
            .find_by_id(&ammunition_type_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionType not found".to_string()))?;

        if ammunition_type.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this ammunition type".to_string(),
            ));
        }

        Ok(AmmunitionTypeResponse::from(ammunition_type))
    }

    /// ユーザーの実包種別一覧を取得
    pub async fn list_ammunition_types(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AmmunitionTypeResponse>, AppError> {
        let ammunition_types = self
            .ammunition_type_repository
            .find_by_user_id(&user_id)
            .await?;

        let responses = ammunition_types
            .into_iter()
            .map(AmmunitionTypeResponse::from)
            .collect();

        Ok(responses)
    }

    /// 実包種別を更新
    pub async fn update_ammunition_type(
        &self,
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
        request: UpdateAmmunitionTypeRequest,
    ) -> Result<AmmunitionTypeResponse, AppError> {
        let mut ammunition_type = self
            .ammunition_type_repository
            .find_by_id(&ammunition_type_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionType not found".to_string()))?;

        if ammunition_type.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to update this ammunition type".to_string(),
            ));
        }

        ammunition_type.update(request.name, request.caliber, request.shot_size, request.is_slug);

        self.ammunition_type_repository
            .save(&ammunition_type)
            .await?;

        Ok(AmmunitionTypeResponse::from(ammunition_type))
    }

    /// 実包種別を削除（論理削除）
    pub async fn delete_ammunition_type(
        &self,
        user_id: UserId,
        ammunition_type_id: AmmunitionTypeId,
    ) -> Result<(), AppError> {
        let ammunition_type = self
            .ammunition_type_repository
            .find_by_id(&ammunition_type_id)
            .await?
            .ok_or_else(|| AppError::NotFound("AmmunitionType not found".to_string()))?;

        if ammunition_type.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this ammunition type".to_string(),
            ));
        }

        self.ammunition_type_repository
            .delete(&ammunition_type_id)
            .await?;

        Ok(())
    }
}
