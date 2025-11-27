use async_trait::async_trait;
use domain::entities::AmmunitionType;
use domain::repositories::AmmunitionTypeRepository;
use domain::value_objects::{AmmunitionTypeId, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};

/// PostgreSQL実装のAmmunitionTypeRepository
#[derive(Clone)]
pub struct PostgresAmmunitionTypeRepository {
    pool: Pool<Postgres>,
}

impl PostgresAmmunitionTypeRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AmmunitionTypeRepository for PostgresAmmunitionTypeRepository {
    async fn find_by_id(
        &self,
        id: &AmmunitionTypeId,
    ) -> Result<Option<AmmunitionType>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, name, caliber, shot_size, is_slug,
                   created_at, updated_at, deleted_at
            FROM ammunition_types
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let ammunition_type = AmmunitionType {
                    id: AmmunitionTypeId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    name: row.get("name"),
                    caliber: row.get("caliber"),
                    shot_size: row.get("shot_size"),
                    is_slug: row.get("is_slug"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(ammunition_type))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<AmmunitionType>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, name, caliber, shot_size, is_slug,
                   created_at, updated_at, deleted_at
            FROM ammunition_types
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY caliber, name
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_types = rows
            .iter()
            .map(|row| AmmunitionType {
                id: AmmunitionTypeId::from_uuid(row.get("id")),
                user_id: UserId::from_uuid(row.get("user_id")),
                name: row.get("name"),
                caliber: row.get("caliber"),
                shot_size: row.get("shot_size"),
                is_slug: row.get("is_slug"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            })
            .collect();

        Ok(ammunition_types)
    }

    async fn find_by_user_id_and_caliber(
        &self,
        user_id: &UserId,
        caliber: &str,
    ) -> Result<Vec<AmmunitionType>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, name, caliber, shot_size, is_slug,
                   created_at, updated_at, deleted_at
            FROM ammunition_types
            WHERE user_id = $1 AND caliber = $2 AND deleted_at IS NULL
            ORDER BY name
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(caliber)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_types = rows
            .iter()
            .map(|row| AmmunitionType {
                id: AmmunitionTypeId::from_uuid(row.get("id")),
                user_id: UserId::from_uuid(row.get("user_id")),
                name: row.get("name"),
                caliber: row.get("caliber"),
                shot_size: row.get("shot_size"),
                is_slug: row.get("is_slug"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            })
            .collect();

        Ok(ammunition_types)
    }

    async fn save(&self, ammunition_type: &AmmunitionType) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO ammunition_types (
                id, user_id, name, caliber, shot_size, is_slug,
                created_at, updated_at, deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                caliber = EXCLUDED.caliber,
                shot_size = EXCLUDED.shot_size,
                is_slug = EXCLUDED.is_slug,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
        )
        .bind(ammunition_type.id.as_uuid())
        .bind(ammunition_type.user_id.as_uuid())
        .bind(&ammunition_type.name)
        .bind(&ammunition_type.caliber)
        .bind(&ammunition_type.shot_size)
        .bind(&ammunition_type.is_slug)
        .bind(&ammunition_type.created_at)
        .bind(&ammunition_type.updated_at)
        .bind(&ammunition_type.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &AmmunitionTypeId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE ammunition_types
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("AmmunitionType not found".to_string()));
        }

        Ok(())
    }
}
