use crate::dto::firearm_dto::{CreateFirearmRequest, FirearmResponse, UpdateFirearmRequest};
use domain::entities::Firearm;
use domain::repositories::FirearmRepository;
use domain::value_objects::{FirearmId, UserId};
use shared::error::AppError;
use std::sync::Arc;

/// 銃砲ユースケース
pub struct FirearmUsecases {
    firearm_repository: Arc<dyn FirearmRepository>,
}

impl FirearmUsecases {
    pub fn new(firearm_repository: Arc<dyn FirearmRepository>) -> Self {
        Self {
            firearm_repository,
        }
    }

    /// 銃砲を作成
    pub async fn create_firearm(
        &self,
        user_id: UserId,
        request: CreateFirearmRequest,
    ) -> Result<FirearmResponse, AppError> {
        let firearm_type = request
            .parse_firearm_type()
            .map_err(|e| AppError::ValidationError(e))?;

        let firearm = Firearm::new(
            user_id,
            firearm_type,
            request.name,
            request.gun_number,
            request.permit_number,
            request.caliber,
            request.notes,
        );

        self.firearm_repository.save(&firearm).await?;

        Ok(FirearmResponse::from(firearm))
    }

    /// 銃砲を取得
    pub async fn get_firearm(
        &self,
        user_id: UserId,
        firearm_id: FirearmId,
    ) -> Result<FirearmResponse, AppError> {
        let firearm = self
            .firearm_repository
            .find_by_id(&firearm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Firearm not found".to_string()))?;

        // ユーザーの所有確認
        if firearm.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this firearm".to_string(),
            ));
        }

        Ok(FirearmResponse::from(firearm))
    }

    /// ユーザーの銃砲一覧を取得
    pub async fn list_firearms(&self, user_id: UserId) -> Result<Vec<FirearmResponse>, AppError> {
        let firearms = self.firearm_repository.find_by_user_id(&user_id).await?;

        let responses = firearms
            .into_iter()
            .map(FirearmResponse::from)
            .collect();

        Ok(responses)
    }

    /// 銃砲を更新
    pub async fn update_firearm(
        &self,
        user_id: UserId,
        firearm_id: FirearmId,
        request: UpdateFirearmRequest,
    ) -> Result<FirearmResponse, AppError> {
        let mut firearm = self
            .firearm_repository
            .find_by_id(&firearm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Firearm not found".to_string()))?;

        // ユーザーの所有確認
        if firearm.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to update this firearm".to_string(),
            ));
        }

        firearm.update(
            request.name,
            request.gun_number,
            request.permit_number,
            request.caliber,
            request.notes,
        );

        self.firearm_repository.save(&firearm).await?;

        Ok(FirearmResponse::from(firearm))
    }

    /// 銃砲を削除（論理削除）
    pub async fn delete_firearm(
        &self,
        user_id: UserId,
        firearm_id: FirearmId,
    ) -> Result<(), AppError> {
        let firearm = self
            .firearm_repository
            .find_by_id(&firearm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Firearm not found".to_string()))?;

        // ユーザーの所有確認
        if firearm.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this firearm".to_string(),
            ));
        }

        self.firearm_repository.delete(&firearm_id).await?;

        Ok(())
    }
}
