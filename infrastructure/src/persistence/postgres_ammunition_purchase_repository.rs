use async_trait::async_trait;
use chrono::NaiveDate;
use domain::entities::AmmunitionPurchase;
use domain::repositories::AmmunitionPurchaseRepository;
use domain::value_objects::{AmmunitionPurchaseId, AmmunitionTypeId, FirearmId, UserId};
use shared::error::AppError;
use sqlx::{Pool, Postgres, Row};

/// PostgreSQL実装のAmmunitionPurchaseRepository
#[derive(Clone)]
pub struct PostgresAmmunitionPurchaseRepository {
    pool: Pool<Postgres>,
}

impl PostgresAmmunitionPurchaseRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AmmunitionPurchaseRepository for PostgresAmmunitionPurchaseRepository {
    async fn find_by_id(
        &self,
        id: &AmmunitionPurchaseId,
    ) -> Result<Option<AmmunitionPurchase>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, purchase_date,
                   supplier, quantity, price, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_purchases
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let firearm_id: Option<uuid::Uuid> = row.get("firearm_id");
                let ammunition_purchase = AmmunitionPurchase {
                    id: AmmunitionPurchaseId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    purchase_date: row.get("purchase_date"),
                    supplier: row.get("supplier"),
                    quantity: row.get("quantity"),
                    price: row.get("price"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                };
                Ok(Some(ammunition_purchase))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<AmmunitionPurchase>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, purchase_date,
                   supplier, quantity, price, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_purchases
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY purchase_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_purchases = rows
            .iter()
            .map(|row| {
                let firearm_id: Option<uuid::Uuid> = row.get("firearm_id");
                AmmunitionPurchase {
                    id: AmmunitionPurchaseId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    purchase_date: row.get("purchase_date"),
                    supplier: row.get("supplier"),
                    quantity: row.get("quantity"),
                    price: row.get("price"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                }
            })
            .collect();

        Ok(ammunition_purchases)
    }

    async fn find_by_user_id_and_date_range(
        &self,
        user_id: &UserId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AmmunitionPurchase>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, purchase_date,
                   supplier, quantity, price, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_purchases
            WHERE user_id = $1 AND purchase_date >= $2 AND purchase_date <= $3
                  AND deleted_at IS NULL
            ORDER BY purchase_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_purchases = rows
            .iter()
            .map(|row| {
                let firearm_id: Option<uuid::Uuid> = row.get("firearm_id");
                AmmunitionPurchase {
                    id: AmmunitionPurchaseId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    purchase_date: row.get("purchase_date"),
                    supplier: row.get("supplier"),
                    quantity: row.get("quantity"),
                    price: row.get("price"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                }
            })
            .collect();

        Ok(ammunition_purchases)
    }

    async fn find_by_user_id_and_ammunition_type_id(
        &self,
        user_id: &UserId,
        ammunition_type_id: &AmmunitionTypeId,
    ) -> Result<Vec<AmmunitionPurchase>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, ammunition_type_id, firearm_id, purchase_date,
                   supplier, quantity, price, notes,
                   created_at, updated_at, deleted_at
            FROM ammunition_purchases
            WHERE user_id = $1 AND ammunition_type_id = $2 AND deleted_at IS NULL
            ORDER BY purchase_date DESC
            "#,
        )
        .bind(user_id.as_uuid())
        .bind(ammunition_type_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let ammunition_purchases = rows
            .iter()
            .map(|row| {
                let firearm_id: Option<uuid::Uuid> = row.get("firearm_id");
                AmmunitionPurchase {
                    id: AmmunitionPurchaseId::from_uuid(row.get("id")),
                    user_id: UserId::from_uuid(row.get("user_id")),
                    ammunition_type_id: AmmunitionTypeId::from_uuid(row.get("ammunition_type_id")),
                    firearm_id: firearm_id.map(FirearmId::from_uuid),
                    purchase_date: row.get("purchase_date"),
                    supplier: row.get("supplier"),
                    quantity: row.get("quantity"),
                    price: row.get("price"),
                    notes: row.get("notes"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                }
            })
            .collect();

        Ok(ammunition_purchases)
    }

    async fn save(&self, ammunition_purchase: &AmmunitionPurchase) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO ammunition_purchases (
                id, user_id, ammunition_type_id, firearm_id, purchase_date,
                supplier, quantity, price, notes,
                created_at, updated_at, deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                ammunition_type_id = EXCLUDED.ammunition_type_id,
                firearm_id = EXCLUDED.firearm_id,
                purchase_date = EXCLUDED.purchase_date,
                supplier = EXCLUDED.supplier,
                quantity = EXCLUDED.quantity,
                price = EXCLUDED.price,
                notes = EXCLUDED.notes,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
        )
        .bind(ammunition_purchase.id.as_uuid())
        .bind(ammunition_purchase.user_id.as_uuid())
        .bind(ammunition_purchase.ammunition_type_id.as_uuid())
        .bind(ammunition_purchase.firearm_id.as_ref().map(|id| id.as_uuid()))
        .bind(&ammunition_purchase.purchase_date)
        .bind(&ammunition_purchase.supplier)
        .bind(&ammunition_purchase.quantity)
        .bind(&ammunition_purchase.price)
        .bind(&ammunition_purchase.notes)
        .bind(&ammunition_purchase.created_at)
        .bind(&ammunition_purchase.updated_at)
        .bind(&ammunition_purchase.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &AmmunitionPurchaseId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE ammunition_purchases
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "AmmunitionPurchase not found".to_string(),
            ));
        }

        Ok(())
    }
}
