use async_trait::async_trait;
use chrono::NaiveDate;
use domain::entities::AmmunitionUsage;
use domain::repositories::AmmunitionUsageRepository;
use domain::value_objects::{AmmunitionTypeId, AmmunitionUsageId, FirearmId, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

/// PostgreSQL実装のAmmunitionUsageRepository
#[derive(Clone)]
pub struct PostgresAmmunitionUsageRepository {
    pool: Pool<Postgres>,
}

impl PostgresAmmunitionUsageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AmmunitionUsageRepository for PostgresAmmunitionUsageRepository {
    async fn find_by_id(
        &self,
        id: &AmmunitionUsageId,
    ) -> Result<Option<AmmunitionUsage>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, hunting_record_id,
                   usage_date, location, quantity_used, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_usages
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let firearm_id: Option<Uuid> = row.get("firearm_id");
                let ammunition_usage = AmmunitionUsage {
                    id: AmmunitionUsageId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    hunting_record_id: row.get("hunting_record_id"),
                    usage_date: row.get("usage_date"),
                    location: row.get("location"),
                    quantity_used: row.get("quantity_used"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(ammunition_usage))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionUsage>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, hunting_record_id,
                   usage_date, location, quantity_used, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_usages
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY usage_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_usages = rows
            .iter()
            .map(|row| {
                let firearm_id: Option<Uuid> = row.get("firearm_id");
                AmmunitionUsage {
                    id: AmmunitionUsageId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    hunting_record_id: row.get("hunting_record_id"),
                    usage_date: row.get("usage_date"),
                    location: row.get("location"),
                    quantity_used: row.get("quantity_used"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                }
            })
            .collect();

        Ok(ammunition_usages)
    }

    async fn find_by_user_id_and_date_range(
        &self,
        user_id: &UserId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AmmunitionUsage>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, hunting_record_id,
                   usage_date, location, quantity_used, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_usages
            WHERE user_id = $1 AND usage_date >= $2 AND usage_date <= $3
                  AND deleted_at IS NULL
            ORDER BY usage_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_usages = rows
            .iter()
            .map(|row| {
                let firearm_id: Option<Uuid> = row.get("firearm_id");
                AmmunitionUsage {
                    id: AmmunitionUsageId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    hunting_record_id: row.get("hunting_record_id"),
                    usage_date: row.get("usage_date"),
                    location: row.get("location"),
                    quantity_used: row.get("quantity_used"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                }
            })
            .collect();

        Ok(ammunition_usages)
    }

    async fn find_by_user_id_and_ammunition_type_id(
        &self,
        user_id: &UserId,
        ammunition_type_id: &AmmunitionTypeId,
    ) -> Result<Vec<AmmunitionUsage>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, hunting_record_id,
                   usage_date, location, quantity_used, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_usages
            WHERE user_id = $1 AND ammunition_type_id = $2 AND deleted_at IS NULL
            ORDER BY usage_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(ammunition_type_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_usages = rows
            .iter()
            .map(|row| {
                let firearm_id: Option<Uuid> = row.get("firearm_id");
                AmmunitionUsage {
                    id: AmmunitionUsageId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    hunting_record_id: row.get("hunting_record_id"),
                    usage_date: row.get("usage_date"),
                    location: row.get("location"),
                    quantity_used: row.get("quantity_used"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                }
            })
            .collect();

        Ok(ammunition_usages)
    }

    async fn save(&self, ammunition_usage: &AmmunitionUsage) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO ammunition_usages (
                id, user_id, ammunition_type_id, firearm_id, hunting_record_id,
                usage_date, location, quantity_used, notes,
                created_at, updated_at, deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                ammunition_type_id = EXCLUDED.ammunition_type_id,
                firearm_id = EXCLUDED.firearm_id,
                hunting_record_id = EXCLUDED.hunting_record_id,
                usage_date = EXCLUDED.usage_date,
                location = EXCLUDED.location,
                quantity_used = EXCLUDED.quantity_used,
                notes = EXCLUDED.notes,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
        )
        .bind(ammunition_usage.id.as_uuid())
        .bind(ammunition_usage.user_id.as_uuid())
        .bind(ammunition_usage.ammunition_type_id.as_uuid())
        .bind(ammunition_usage.firearm_id.as_ref().map(|id| id.as_uuid()))
        .bind(ammunition_usage.hunting_record_id)
        .bind(ammunition_usage.usage_date)
        .bind(&ammunition_usage.location)
        .bind(ammunition_usage.quantity_used)
        .bind(&ammunition_usage.notes)
        .bind(ammunition_usage.created_at)
        .bind(ammunition_usage.updated_at)
        .bind(ammunition_usage.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &AmmunitionUsageId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE ammunition_usages
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("AmmunitionUsage not found".to_string()));
        }

        Ok(())
    }
}
