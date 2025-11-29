use async_trait::async_trait;
use domain::entities::AmmunitionLimit;
use domain::repositories::AmmunitionLimitRepository;
use domain::value_objects::{AmmunitionLimitId, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};

/// PostgreSQL実装のAmmunitionLimitRepository
#[derive(Clone)]
pub struct PostgresAmmunitionLimitRepository {
    pool: Pool<Postgres>,
}

impl PostgresAmmunitionLimitRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AmmunitionLimitRepository for PostgresAmmunitionLimitRepository {
    async fn find_by_id(
        &self,
        id: &AmmunitionLimitId,
    ) -> Result<Option<AmmunitionLimit>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, caliber, max_quantity, created_at, updated_at
            FROM ammunition_limits
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let ammunition_limit = AmmunitionLimit {
                    id: AmmunitionLimitId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    caliber: row.get("caliber"),
                    max_quantity: row.get("max_quantity"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(ammunition_limit))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionLimit>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, caliber, max_quantity, created_at, updated_at
            FROM ammunition_limits
            WHERE user_id = $1
            ORDER BY caliber
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_limits = rows
            .iter()
            .map(|row| AmmunitionLimit {
                id: AmmunitionLimitId::from_uuid(row.get("id")),
                user_id: UserId::from_uuid(row.get("user_id")),
                caliber: row.get("caliber"),
                max_quantity: row.get("max_quantity"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(ammunition_limits)
    }

    async fn find_by_user_id_and_caliber(
        &self,
        user_id: &UserId,
        caliber: &str,
    ) -> Result<Option<AmmunitionLimit>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, caliber, max_quantity, created_at, updated_at
            FROM ammunition_limits
            WHERE user_id = $1 AND caliber = $2
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(caliber)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let ammunition_limit = AmmunitionLimit {
                    id: AmmunitionLimitId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    caliber: row.get("caliber"),
                    max_quantity: row.get("max_quantity"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(ammunition_limit))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, ammunition_limit: &AmmunitionLimit) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO ammunition_limits (
                id, user_id, caliber, max_quantity, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id, caliber) DO UPDATE SET
                max_quantity = EXCLUDED.max_quantity,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(ammunition_limit.id.as_uuid())
        .bind(ammunition_limit.user_id.as_uuid())
        .bind(&ammunition_limit.caliber)
        .bind(ammunition_limit.max_quantity)
        .bind(ammunition_limit.created_at)
        .bind(ammunition_limit.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &AmmunitionLimitId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM ammunition_limits
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("AmmunitionLimit not found".to_string()));
        }

        Ok(())
    }
}
