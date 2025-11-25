# Karilog アーキテクチャ設計書

## 1. 概要

### 1.1 アーキテクチャパターン
**DDD（ドメイン駆動設計）+ クリーンアーキテクチャ**

ビジネスロジックをドメイン層に集約し、外部依存を分離することで、テスタビリティと保守性を確保する。

### 1.2 技術スタック
| レイヤー | 技術 |
|----------|------|
| 言語 | Rust 1.75+ |
| Webフレームワーク | Axum 0.7 |
| ORM | SQLx 0.8 |
| データベース | PostgreSQL 15+ |
| 認証 | JWT (jsonwebtoken) |
| パスワードハッシュ | Argon2id |
| メール送信 | AWS SES |
| PDF生成 | genpdf |

---

## 2. レイヤー構成

### 2.1 ディレクトリ構造

```
karilog/
├── Cargo.toml              # Workspace定義
├── Cargo.lock
├── CLAUDE.md               # 開発ガイド
├── README.md
├── LICENSE
├── .env.example
├── migrations/             # SQLxマイグレーション
│
├── domain/                 # ドメイン層
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── entities/       # エンティティ
│       ├── value_objects/  # 値オブジェクト
│       └── repositories/   # リポジトリインターフェース
│
├── application/            # アプリケーション層
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── use_cases/      # ユースケース
│       ├── dto/            # データ転送オブジェクト
│       └── services/       # ドメインサービス
│
├── infrastructure/         # インフラストラクチャ層
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── repositories/   # リポジトリ実装
│       ├── services/       # 外部サービス実装
│       │   ├── auth/       # 認証サービス
│       │   ├── email/      # メールサービス
│       │   └── pdf/        # PDF生成サービス
│       └── database/       # DB接続
│
├── presentation/           # プレゼンテーション層
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── handlers/       # HTTPハンドラ
│       ├── requests/       # リクエストDTO
│       ├── responses/      # レスポンスDTO
│       ├── middleware/     # ミドルウェア
│       └── routes.rs       # ルーティング
│
├── shared/                 # 共通モジュール
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── config.rs       # 設定
│       └── error.rs        # エラー定義
│
├── container/              # DIコンテナ
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── modules.rs      # 依存性注入
│
└── src/
    └── main.rs             # エントリポイント
```

### 2.2 依存関係

```
┌─────────────────────────────────────────────────────────┐
│                      main.rs                            │
│                         │                               │
│                         ▼                               │
│                    container                            │
│            ┌───────────┼───────────┐                   │
│            ▼           ▼           ▼                   │
│     presentation  application  infrastructure          │
│            │           │           │                   │
│            └─────┬─────┘           │                   │
│                  ▼                 │                   │
│               domain ◄─────────────┘                   │
│                  │                                     │
│                  ▼                                     │
│               shared                                   │
└─────────────────────────────────────────────────────────┘

依存の方向:
- presentation → application → domain
- infrastructure → domain
- 全レイヤー → shared
- container → 全レイヤー（DIのため）
```

---

## 3. 各レイヤーの責務

### 3.1 Domain層（ドメイン層）

**責務**: ビジネスルールとドメイン知識の表現

**含まれるもの**:
- **エンティティ**: 識別子を持つドメインオブジェクト
- **値オブジェクト**: 不変で識別子を持たないオブジェクト
- **リポジトリインターフェース**: データ永続化の抽象
- **ドメインイベント**: （将来的に）

**依存**: shared のみ

```rust
// エンティティの例
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    // ...
}

impl User {
    pub fn is_locked(&self) -> bool {
        // ビジネスロジック
    }
}

// リポジトリインターフェースの例
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn save(&self, user: &User) -> AppResult<()>;
}
```

### 3.2 Application層（アプリケーション層）

**責務**: ユースケースの実装、トランザクション管理

**含まれるもの**:
- **ユースケース**: アプリケーションの機能単位
- **DTO**: レイヤー間のデータ転送
- **アプリケーションサービス**: 複数エンティティにまたがる処理

**依存**: domain, shared

```rust
// ユースケースの例
pub struct RegisterUserUseCase<R: UserRepository> {
    user_repository: R,
    password_hasher: Arc<dyn PasswordHasher>,
    email_service: Arc<dyn EmailService>,
}

impl<R: UserRepository> RegisterUserUseCase<R> {
    pub async fn execute(&self, input: RegisterUserInput) -> AppResult<RegisterUserOutput> {
        // 1. バリデーション
        // 2. 重複チェック
        // 3. パスワードハッシュ化
        // 4. ユーザー作成
        // 5. 確認メール送信
    }
}
```

### 3.3 Infrastructure層（インフラストラクチャ層）

**責務**: 外部システムとの連携実装

**含まれるもの**:
- **リポジトリ実装**: SQLxを使ったDB操作
- **外部サービス実装**: AWS SES、PDF生成など
- **認証サービス**: JWT生成・検証

**依存**: domain, shared

```rust
// リポジトリ実装の例
pub struct PgUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
```

### 3.4 Presentation層（プレゼンテーション層）

**責務**: HTTPリクエスト/レスポンスの処理

**含まれるもの**:
- **ハンドラ**: Axumのリクエストハンドラ
- **リクエストDTO**: 入力バリデーション
- **レスポンスDTO**: JSONレスポンス構造
- **ミドルウェア**: 認証、ロギング、エラーハンドリング
- **ルーティング**: エンドポイント定義

**依存**: application, domain, shared

```rust
// ハンドラの例
pub async fn register_user(
    State(use_case): State<Arc<RegisterUserUseCase>>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, AppError> {
    let input = request.into_input()?;
    let output = use_case.execute(input).await?;
    Ok(Json(output.into()))
}
```

### 3.5 Shared層（共通層）

**責務**: 全レイヤーで共有するユーティリティ

**含まれるもの**:
- **設定**: アプリケーション設定
- **エラー**: 共通エラー型
- **ユーティリティ**: 日付処理など

**依存**: 外部クレートのみ

### 3.6 Container層（DIコンテナ層）

**責務**: 依存性の注入と組み立て

**含まれるもの**:
- **モジュール定義**: 各サービスの生成
- **AppState**: Axumで共有する状態

**依存**: 全レイヤー

```rust
// DIコンテナの例
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub register_user_use_case: Arc<RegisterUserUseCase>,
    // ...
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let pool = PgPool::connect(&config.database.url).await?;
        let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
        // ...
    }
}
```

---

## 4. 認証アーキテクチャ

### 4.1 認証フロー

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Client  │───>│  Axum    │───>│  Auth    │───>│   DB     │
│          │    │ Handler  │    │ Service  │    │          │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
     │               │               │               │
     │  POST /login  │               │               │
     │──────────────>│               │               │
     │               │  validate     │               │
     │               │──────────────>│               │
     │               │               │  find user    │
     │               │               │──────────────>│
     │               │               │<──────────────│
     │               │               │               │
     │               │  verify pass  │               │
     │               │<──────────────│               │
     │               │               │               │
     │               │  gen tokens   │               │
     │               │──────────────>│               │
     │               │               │  save refresh │
     │               │               │──────────────>│
     │               │<──────────────│<──────────────│
     │  tokens       │               │               │
     │<──────────────│               │               │
```

### 4.2 JWT構造

**アクセストークン**:
```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "exp": 1234567890,
  "iat": 1234567890,
  "type": "access"
}
```

**リフレッシュトークン**:
```json
{
  "sub": "user-uuid",
  "jti": "token-uuid",
  "exp": 1234567890,
  "iat": 1234567890,
  "type": "refresh"
}
```

### 4.3 認証ミドルウェア

```rust
pub async fn auth_middleware(
    State(auth_service): State<Arc<dyn AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer_token(&request)?;
    let claims = auth_service.verify_access_token(&token)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
```

---

## 5. エラーハンドリング

### 5.1 エラー型

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("認証が必要です")]
    Unauthorized,

    #[error("指定されたリソースが見つかりません")]
    NotFound,

    #[error("入力内容に誤りがあります: {0}")]
    Validation(String),

    // ...
}
```

### 5.2 HTTPレスポンス変換

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = Json(ErrorResponse {
            code: self.code().to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}
```

---

## 6. テスト戦略

### 6.1 テストピラミッド

```
        ┌───────────┐
        │   E2E     │  少数
        │  Tests    │
        ├───────────┤
        │Integration│  中程度
        │  Tests    │
        ├───────────┤
        │   Unit    │  多数
        │  Tests    │
        └───────────┘
```

### 6.2 各レイヤーのテスト

| レイヤー | テスト種類 | ツール |
|----------|------------|--------|
| Domain | 単体テスト | 標準テスト |
| Application | 単体テスト + モック | mockall |
| Infrastructure | 統合テスト | testcontainers |
| Presentation | E2Eテスト | axum-test |

### 6.3 モックの使用

```rust
#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_register_user() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_find_by_email()
            .returning(|_| Ok(None));
        mock_repo
            .expect_save()
            .returning(|_| Ok(()));

        let use_case = RegisterUserUseCase::new(Arc::new(mock_repo), ...);
        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }
}
```

---

## 7. 将来の拡張性

### 7.1 イベント駆動への拡張
- ドメインイベントの導入
- メッセージキュー（SQS等）との連携

### 7.2 CQRS
- 読み取り用の専用モデル導入
- 複雑なクエリのパフォーマンス最適化

### 7.3 マイクロサービス化
- 現在のモジュール境界を維持
- 必要に応じてサービス分割可能

---

## 8. 改訂履歴

| バージョン | 日付 | 内容 |
|------------|------|------|
| 1.0 | 2025-01-XX | 初版作成 |