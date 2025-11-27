use async_trait::async_trait;
use chrono::NaiveDate;
use domain::entities::HuntingRecord;
use domain::repositories::HuntingRecordRepository;
use domain::value_objects::{HuntingRecordId, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};

/// PostgreSQL実装のHuntingRecordRepository
#[derive(Clone)]
pub struct PostgresHuntingRecordRepository {
    pool: Pool<Postgres>,
}

impl PostgresHuntingRecordRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HuntingRecordRepository for PostgresHuntingRecordRepository {
    async fn find_by_id(&self, id: &HuntingRecordId) -> Result<Option<HuntingRecord>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, hunting_date, location, is_planned, notes,
                   created_at, updated_at, deleted_at
            FROM hunting_records
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let record = HuntingRecord {
                    id: HuntingRecordId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    hunting_date: row.get("hunting_date"),
                    location: row.get("location"),
                    is_planned: row.get("is_planned"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<HuntingRecord>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, hunting_date, location, is_planned, notes,
                   created_at, updated_at, deleted_at
            FROM hunting_records
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY hunting_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut records = Vec::new();
        for row in rows {
            let record = HuntingRecord {
                id: HuntingRecordId::from_uuid(row.get("id")),
                user_id: UserId::from_uuid(row.get("user_id")),
                hunting_date: row.get("hunting_date"),
                location: row.get("location"),
                is_planned: row.get("is_planned"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            };
            records.push(record);
        }

        Ok(records)
    }

    async fn find_by_user_and_date_range(
        &self,
        user_id: &UserId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<HuntingRecord>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, hunting_date, location, is_planned, notes,
                   created_at, updated_at, deleted_at
            FROM hunting_records
            WHERE user_id = $1 AND hunting_date BETWEEN $2 AND $3 AND deleted_at IS NULL
            ORDER BY hunting_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut records = Vec::new();
        for row in rows {
            let record = HuntingRecord {
                id: HuntingRecordId::from_uuid(row.get("id")),
                user_id: UserId::from_uuid(row.get("user_id")),
                hunting_date: row.get("hunting_date"),
                location: row.get("location"),
                is_planned: row.get("is_planned"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            };
            records.push(record);
        }

        Ok(records)
    }

    async fn save(&self, record: &HuntingRecord) -> Result<(), AppError> {
        // INSERT ... ON CONFLICT ... DO UPDATE パターンを使用（upsert）
        sqlx::query(
            r#"
            INSERT INTO hunting_records (
                id, user_id, hunting_date, location, is_planned, notes,
                created_at, updated_at, deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                hunting_date = EXCLUDED.hunting_date,
                location = EXCLUDED.location,
                is_planned = EXCLUDED.is_planned,
                notes = EXCLUDED.notes,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
        )
        .bind(record.id.as_uuid())
        .bind(record.user_id.as_uuid())
        .bind(record.hunting_date)
        .bind(&record.location)
        .bind(record.is_planned)
        .bind(&record.notes)
        .bind(record.created_at)
        .bind(record.updated_at)
        .bind(record.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &HuntingRecordId) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE hunting_records
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
