use async_trait::async_trait;
use domain::entities::{Firearm, FirearmType};
use domain::repositories::FirearmRepository;
use domain::value_objects::{FirearmId, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};

/// PostgreSQL実装のFirearmRepository
#[derive(Clone)]
pub struct PostgresFirearmRepository {
    pool: Pool<Postgres>,
}

impl PostgresFirearmRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FirearmRepository for PostgresFirearmRepository {
    async fn find_by_id(&self, id: &FirearmId) -> Result<Option<Firearm>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, firearm_type, name, gun_number, permit_number,
                   caliber, notes, created_at, updated_at, deleted_at
            FROM firearms
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let firearm_type_str: String = row.get("firearm_type");
                let firearm_type = match firearm_type_str.as_str() {
                    "shotgun" => FirearmType::Shotgun,
                    "rifle" => FirearmType::Rifle,
                    _ => {
                        return Err(AppError::ValidationError(format!(
                            "Invalid firearm type: {}",
                            firearm_type_str
                        )))
                    }
                };

                let firearm = Firearm {
                    id: FirearmId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    firearm_type,
                    name: row.get("name"),
                    gun_number: row.get("gun_number"),
                    permit_number: row.get("permit_number"),
                    caliber: row.get("caliber"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(firearm))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Firearm>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, firearm_type, name, gun_number, permit_number,
                   caliber, notes, created_at, updated_at, deleted_at
            FROM firearms
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut firearms = Vec::new();
        for row in rows {
            let firearm_type_str: String = row.get("firearm_type");
            let firearm_type = match firearm_type_str.as_str() {
                "shotgun" => FirearmType::Shotgun,
                "rifle" => FirearmType::Rifle,
                _ => {
                    return Err(AppError::ValidationError(format!(
                        "Invalid firearm type: {}",
                        firearm_type_str
                    )))
                }
            };

            let firearm = Firearm {
                id: FirearmId::from_uuid(row.get("id")),
                user_id: UserId::from_uuid(row.get("user_id")),
                firearm_type,
                name: row.get("name"),
                gun_number: row.get("gun_number"),
                permit_number: row.get("permit_number"),
                caliber: row.get("caliber"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            };
            firearms.push(firearm);
        }

        Ok(firearms)
    }

    async fn save(&self, firearm: &Firearm) -> Result<(), AppError> {
        // INSERT ... ON CONFLICT ... DO UPDATE パターンを使用（upsert）
        sqlx::query(
            r#"
            INSERT INTO firearms (
                id, user_id, firearm_type, name, gun_number, permit_number,
                caliber, notes, created_at, updated_at, deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                firearm_type = EXCLUDED.firearm_type,
                name = EXCLUDED.name,
                gun_number = EXCLUDED.gun_number,
                permit_number = EXCLUDED.permit_number,
                caliber = EXCLUDED.caliber,
                notes = EXCLUDED.notes,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
        )
        .bind(firearm.id.as_uuid())
        .bind(firearm.user_id.as_uuid())
        .bind(firearm.firearm_type.as_str())
        .bind(&firearm.name)
        .bind(&firearm.gun_number)
        .bind(&firearm.permit_number)
        .bind(&firearm.caliber)
        .bind(&firearm.notes)
        .bind(&firearm.created_at)
        .bind(&firearm.updated_at)
        .bind(&firearm.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &FirearmId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE firearms
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Firearm not found".to_string()));
        }

        Ok(())
    }
}
