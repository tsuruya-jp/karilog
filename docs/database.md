# Karilog データベース設計書

## 1. 概要

### 1.1 データベース
- **DBMS**: PostgreSQL 15以上
- **文字コード**: UTF-8
- **タイムゾーン**: UTC

### 1.2 設計方針
- 論理削除（soft delete）を採用
- UUIDを主キーとして使用
- タイムスタンプはUTCで保存
- マルチテナンシー対応（user_idで分離）

---

## 2. ER図

```
┌─────────────┐       ┌─────────────────────┐
│   users     │       │   refresh_tokens    │
├─────────────┤       ├─────────────────────┤
│ id (PK)     │──────<│ user_id (FK)        │
│ email       │       │ token_hash          │
│ ...         │       │ ...                 │
└─────────────┘       └─────────────────────┘
       │
       │
       ├──────────────────────┬─────────────────────┐
       │                      │                     │
       ▼                      ▼                     ▼
┌─────────────┐       ┌─────────────────┐   ┌─────────────────┐
│  firearms   │       │ammunition_types │   │ammunition_limits│
├─────────────┤       ├─────────────────┤   ├─────────────────┤
│ id (PK)     │       │ id (PK)         │   │ id (PK)         │
│ user_id(FK) │       │ user_id (FK)    │   │ user_id (FK)    │
│ ...         │       │ ...             │   │ caliber         │
└─────────────┘       └─────────────────┘   │ max_quantity    │
       │                      │             └─────────────────┘
       │                      │
       │    ┌─────────────────┴─────────────────┐
       │    │                                   │
       ▼    ▼                                   ▼
┌─────────────────────┐               ┌─────────────────────┐
│ammunition_purchases │               │ ammunition_usages   │
├─────────────────────┤               ├─────────────────────┤
│ id (PK)             │               │ id (PK)             │
│ user_id (FK)        │               │ user_id (FK)        │
│ ammunition_type_id  │               │ ammunition_type_id  │
│ firearm_id (FK)     │               │ firearm_id (FK)     │
│ ...                 │               │ hunting_record_id   │──┐
└─────────────────────┘               │ ...                 │  │
                                      └─────────────────────┘  │
                                                               │
       ┌───────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────┐
│ hunting_records │
├─────────────────┤
│ id (PK)         │
│ user_id (FK)    │
│ ...             │
└─────────────────┘
```

---

## 3. テーブル定義

### 3.1 users（ユーザー）

ユーザーアカウント情報を管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| email | VARCHAR(255) | NO | - | メールアドレス（UNIQUE） |
| password_hash | VARCHAR(255) | NO | - | パスワードハッシュ（Argon2id） |
| name | VARCHAR(100) | YES | NULL | ユーザー名 |
| email_verified | BOOLEAN | NO | false | メール確認済みフラグ |
| email_verification_token | VARCHAR(255) | YES | NULL | メール確認トークン |
| email_verification_expires_at | TIMESTAMPTZ | YES | NULL | メール確認トークン有効期限 |
| password_reset_token | VARCHAR(255) | YES | NULL | パスワードリセットトークン |
| password_reset_expires_at | TIMESTAMPTZ | YES | NULL | パスワードリセットトークン有効期限 |
| failed_login_attempts | INTEGER | NO | 0 | ログイン失敗回数 |
| locked_until | TIMESTAMPTZ | YES | NULL | アカウントロック解除日時 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |
| deleted_at | TIMESTAMPTZ | YES | NULL | 削除日時（論理削除） |

**インデックス:**
- `idx_users_email` ON users(email) WHERE deleted_at IS NULL
- `idx_users_email_verification_token` ON users(email_verification_token) WHERE email_verification_token IS NOT NULL
- `idx_users_password_reset_token` ON users(password_reset_token) WHERE password_reset_token IS NOT NULL

---

### 3.2 refresh_tokens（リフレッシュトークン）

JWTリフレッシュトークンを管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| token_hash | VARCHAR(255) | NO | - | トークンハッシュ |
| expires_at | TIMESTAMPTZ | NO | - | 有効期限 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| revoked_at | TIMESTAMPTZ | YES | NULL | 無効化日時 |

**インデックス:**
- `idx_refresh_tokens_user_id` ON refresh_tokens(user_id)
- `idx_refresh_tokens_token_hash` ON refresh_tokens(token_hash) WHERE revoked_at IS NULL

**外部キー:**
- user_id → users(id) ON DELETE CASCADE

---

### 3.3 firearms（銃砲）

銃砲情報を管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| firearm_type | VARCHAR(20) | NO | - | 銃種（shotgun/rifle） |
| name | VARCHAR(200) | NO | - | 銃の名称 |
| gun_number | VARCHAR(100) | NO | - | 銃番号 |
| permit_number | VARCHAR(100) | NO | - | 許可番号 |
| caliber | VARCHAR(50) | YES | NULL | 適合実包の口径 |
| notes | TEXT | YES | NULL | 備考 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |
| deleted_at | TIMESTAMPTZ | YES | NULL | 削除日時（論理削除） |

**インデックス:**
- `idx_firearms_user_id` ON firearms(user_id) WHERE deleted_at IS NULL

**外部キー:**
- user_id → users(id) ON DELETE CASCADE

---

### 3.4 ammunition_types（実包種別）

実包種別マスタ。ユーザーごとに自由登録可能。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| name | VARCHAR(200) | NO | - | 実包名称（例：12番 7.5号） |
| caliber | VARCHAR(50) | NO | - | 口径（例：12番） |
| shot_size | VARCHAR(50) | YES | NULL | 散弾サイズ（例：7.5号） |
| is_slug | BOOLEAN | NO | false | スラッグ弾フラグ |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |
| deleted_at | TIMESTAMPTZ | YES | NULL | 削除日時（論理削除） |

**インデックス:**
- `idx_ammunition_types_user_id` ON ammunition_types(user_id) WHERE deleted_at IS NULL
- `idx_ammunition_types_caliber` ON ammunition_types(user_id, caliber) WHERE deleted_at IS NULL

**外部キー:**
- user_id → users(id) ON DELETE CASCADE

---

### 3.5 ammunition_purchases（実包購入記録）

実包の購入記録を管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| ammunition_type_id | UUID | NO | - | 実包種別ID（FK） |
| firearm_id | UUID | YES | NULL | 銃砲ID（FK） |
| purchase_date | DATE | NO | - | 購入日 |
| supplier | VARCHAR(200) | NO | - | 購入先 |
| quantity | INTEGER | NO | - | 購入数量 |
| price | INTEGER | YES | NULL | 金額 |
| notes | TEXT | YES | NULL | 備考 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |
| deleted_at | TIMESTAMPTZ | YES | NULL | 削除日時（論理削除） |

**インデックス:**
- `idx_ammunition_purchases_user_date` ON ammunition_purchases(user_id, purchase_date) WHERE deleted_at IS NULL
- `idx_ammunition_purchases_ammunition_type` ON ammunition_purchases(ammunition_type_id) WHERE deleted_at IS NULL

**外部キー:**
- user_id → users(id) ON DELETE CASCADE
- ammunition_type_id → ammunition_types(id) ON DELETE RESTRICT
- firearm_id → firearms(id) ON DELETE SET NULL

**制約:**
- CHECK (quantity > 0)
- CHECK (price IS NULL OR price >= 0)

---

### 3.6 ammunition_usages（実包使用記録）

実包の使用記録を管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| ammunition_type_id | UUID | NO | - | 実包種別ID（FK） |
| firearm_id | UUID | YES | NULL | 銃砲ID（FK） |
| hunting_record_id | UUID | YES | NULL | 出猟記録ID（FK） |
| usage_date | DATE | NO | - | 使用日 |
| location | VARCHAR(200) | YES | NULL | 使用場所 |
| quantity_used | INTEGER | NO | - | 使用数量 |
| notes | TEXT | YES | NULL | 備考 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |
| deleted_at | TIMESTAMPTZ | YES | NULL | 削除日時（論理削除） |

**インデックス:**
- `idx_ammunition_usages_user_date` ON ammunition_usages(user_id, usage_date) WHERE deleted_at IS NULL
- `idx_ammunition_usages_ammunition_type` ON ammunition_usages(ammunition_type_id) WHERE deleted_at IS NULL
- `idx_ammunition_usages_hunting_record` ON ammunition_usages(hunting_record_id) WHERE hunting_record_id IS NOT NULL AND deleted_at IS NULL

**外部キー:**
- user_id → users(id) ON DELETE CASCADE
- ammunition_type_id → ammunition_types(id) ON DELETE RESTRICT
- firearm_id → firearms(id) ON DELETE SET NULL
- hunting_record_id → hunting_records(id) ON DELETE SET NULL

**制約:**
- CHECK (quantity_used > 0)

---

### 3.7 ammunition_limits（実包所持許可上限）

口径ごとの所持許可上限を管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| caliber | VARCHAR(50) | NO | - | 口径 |
| max_quantity | INTEGER | NO | - | 許可上限数 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |

**インデックス:**
- `idx_ammunition_limits_user_caliber` ON ammunition_limits(user_id, caliber) UNIQUE

**外部キー:**
- user_id → users(id) ON DELETE CASCADE

**制約:**
- CHECK (max_quantity > 0)
- UNIQUE (user_id, caliber)

---

### 3.8 hunting_records（出猟記録）

出猟予定・実績を管理する。

| カラム名 | データ型 | NULL | デフォルト | 説明 |
|----------|----------|------|------------|------|
| id | UUID | NO | gen_random_uuid() | 主キー |
| user_id | UUID | NO | - | ユーザーID（FK） |
| hunting_date | DATE | NO | - | 出猟日 |
| location | VARCHAR(200) | YES | NULL | 出猟場所 |
| is_planned | BOOLEAN | NO | true | 予定フラグ（true=予定、false=実績） |
| notes | TEXT | YES | NULL | 備考 |
| created_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 作成日時 |
| updated_at | TIMESTAMPTZ | NO | CURRENT_TIMESTAMP | 更新日時 |
| deleted_at | TIMESTAMPTZ | YES | NULL | 削除日時（論理削除） |

**インデックス:**
- `idx_hunting_records_user_date` ON hunting_records(user_id, hunting_date) WHERE deleted_at IS NULL
- `idx_hunting_records_user_planned` ON hunting_records(user_id, is_planned) WHERE deleted_at IS NULL

**外部キー:**
- user_id → users(id) ON DELETE CASCADE

---

## 4. ビュー定義

### 4.1 current_ammunition_stock（実包残数ビュー）

実包種別ごとの現在在庫数を計算する。

```sql
CREATE VIEW current_ammunition_stock AS
SELECT
    at.user_id,
    at.id AS ammunition_type_id,
    at.name AS ammunition_type_name,
    at.caliber,
    COALESCE(SUM(ap.quantity), 0) AS total_purchased,
    COALESCE(SUM(au.quantity_used), 0) AS total_used,
    COALESCE(SUM(ap.quantity), 0) - COALESCE(SUM(au.quantity_used), 0) AS current_stock,
    al.max_quantity,
    CASE
        WHEN al.max_quantity IS NOT NULL THEN
            al.max_quantity - (COALESCE(SUM(ap.quantity), 0) - COALESCE(SUM(au.quantity_used), 0))
        ELSE NULL
    END AS remaining_quota
FROM ammunition_types at
LEFT JOIN ammunition_purchases ap
    ON at.id = ap.ammunition_type_id AND ap.deleted_at IS NULL
LEFT JOIN ammunition_usages au
    ON at.id = au.ammunition_type_id AND au.deleted_at IS NULL
LEFT JOIN ammunition_limits al
    ON at.user_id = al.user_id AND at.caliber = al.caliber
WHERE at.deleted_at IS NULL
GROUP BY at.user_id, at.id, at.name, at.caliber, al.max_quantity;
```

---

## 5. マイグレーション

### 5.1 初期マイグレーション順序

1. `001_create_users.sql` - usersテーブル
2. `002_create_refresh_tokens.sql` - refresh_tokensテーブル
3. `003_create_firearms.sql` - firearmsテーブル
4. `004_create_ammunition_types.sql` - ammunition_typesテーブル
5. `005_create_ammunition_limits.sql` - ammunition_limitsテーブル
6. `006_create_hunting_records.sql` - hunting_recordsテーブル
7. `007_create_ammunition_purchases.sql` - ammunition_purchasesテーブル
8. `008_create_ammunition_usages.sql` - ammunition_usagesテーブル
9. `009_create_views.sql` - ビュー作成

---

## 6. 運用考慮事項

### 6.1 バックアップ
- 日次でフルバックアップ
- WALアーカイブによるポイントインタイムリカバリ対応

### 6.2 パフォーマンス
- 大量データ時はパーティショニング検討（purchase_date、usage_dateで）
- 定期的なVACUUM ANALYZE実行

### 6.3 データ保持
- 論理削除データは物理削除しない（法的要件）
- 必要に応じてアーカイブテーブルへ移動

---

## 7. 改訂履歴

| バージョン | 日付 | 内容 |
|------------|------|------|
| 1.0 | 2025-01-XX | 初版作成 |