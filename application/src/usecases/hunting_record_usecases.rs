use crate::dto::hunting_record_dto::{
    CreateHuntingRecordRequest, HuntingRecordResponse, HuntingStatisticsResponse,
    ListHuntingRecordsQuery, UpdateHuntingRecordRequest,
};
use chrono::{Datelike, Utc};
use domain::entities::HuntingRecord;
use domain::repositories::HuntingRecordRepository;
use domain::value_objects::{HuntingRecordId, UserId};
use shared::error::AppError;
use std::sync::Arc;

/// 出猟記録ユースケース
pub struct HuntingRecordUsecases {
    hunting_record_repository: Arc<dyn HuntingRecordRepository>,
}

impl HuntingRecordUsecases {
    pub fn new(hunting_record_repository: Arc<dyn HuntingRecordRepository>) -> Self {
        Self {
            hunting_record_repository,
        }
    }

    /// 出猟記録を作成
    pub async fn create_hunting_record(
        &self,
        user_id: UserId,
        request: CreateHuntingRecordRequest,
    ) -> Result<HuntingRecordResponse, AppError> {
        let record = HuntingRecord::new(
            user_id,
            request.hunting_date,
            request.location,
            request.is_planned,
            request.notes,
        );

        self.hunting_record_repository.save(&record).await?;

        Ok(HuntingRecordResponse::from(record))
    }

    /// 出猟記録を取得
    pub async fn get_hunting_record(
        &self,
        user_id: UserId,
        record_id: HuntingRecordId,
    ) -> Result<HuntingRecordResponse, AppError> {
        let record = self
            .hunting_record_repository
            .find_by_id(&record_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hunting record not found".to_string()))?;

        // ユーザーの所有確認
        if record.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to access this hunting record".to_string(),
            ));
        }

        Ok(HuntingRecordResponse::from(record))
    }

    /// 出猟記録一覧を取得
    pub async fn list_hunting_records(
        &self,
        user_id: UserId,
        query: ListHuntingRecordsQuery,
    ) -> Result<Vec<HuntingRecordResponse>, AppError> {
        let records = if let (Some(start_date), Some(end_date)) = (query.start_date, query.end_date)
        {
            // 日付範囲指定がある場合
            self.hunting_record_repository
                .find_by_user_and_date_range(&user_id, start_date, end_date)
                .await?
        } else {
            // 全件取得
            self.hunting_record_repository
                .find_by_user_id(&user_id)
                .await?
        };

        // is_plannedでフィルタリング
        let filtered_records: Vec<HuntingRecord> = if let Some(is_planned) = query.is_planned {
            records
                .into_iter()
                .filter(|r| r.is_planned == is_planned)
                .collect()
        } else {
            records
        };

        let responses = filtered_records
            .into_iter()
            .map(HuntingRecordResponse::from)
            .collect();

        Ok(responses)
    }

    /// 出猟記録を更新
    pub async fn update_hunting_record(
        &self,
        user_id: UserId,
        record_id: HuntingRecordId,
        request: UpdateHuntingRecordRequest,
    ) -> Result<HuntingRecordResponse, AppError> {
        let mut record = self
            .hunting_record_repository
            .find_by_id(&record_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hunting record not found".to_string()))?;

        // ユーザーの所有確認
        if record.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to update this hunting record".to_string(),
            ));
        }

        record.update(
            request.hunting_date,
            request.location,
            request.is_planned,
            request.notes,
        );

        self.hunting_record_repository.save(&record).await?;

        Ok(HuntingRecordResponse::from(record))
    }

    /// 出猟記録を削除
    pub async fn delete_hunting_record(
        &self,
        user_id: UserId,
        record_id: HuntingRecordId,
    ) -> Result<(), AppError> {
        let record = self
            .hunting_record_repository
            .find_by_id(&record_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hunting record not found".to_string()))?;

        // ユーザーの所有確認
        if record.user_id != user_id {
            return Err(AppError::Forbidden(
                "You don't have permission to delete this hunting record".to_string(),
            ));
        }

        self.hunting_record_repository.delete(&record_id).await?;

        Ok(())
    }

    /// 出猟統計を取得
    pub async fn get_statistics(
        &self,
        user_id: UserId,
    ) -> Result<HuntingStatisticsResponse, AppError> {
        let all_records = self
            .hunting_record_repository
            .find_by_user_id(&user_id)
            .await?;

        let total_hunting_days = all_records.len() as i64;
        let planned_count = all_records.iter().filter(|r| r.is_planned).count() as i64;
        let completed_count = all_records.iter().filter(|r| !r.is_planned).count() as i64;

        let now = Utc::now();
        let current_year = now.year();
        let current_month = now.month();

        let current_year_count = all_records
            .iter()
            .filter(|r| r.hunting_date.year() == current_year)
            .count() as i64;

        let current_month_count = all_records
            .iter()
            .filter(|r| {
                r.hunting_date.year() == current_year && r.hunting_date.month() == current_month
            })
            .count() as i64;

        Ok(HuntingStatisticsResponse {
            total_hunting_days,
            planned_count,
            completed_count,
            current_year_count,
            current_month_count,
        })
    }
}
