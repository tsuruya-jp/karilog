# Karilog API設計書

## 概要

本ドキュメントはKarilogのAPI仕様を定義します。
詳細な仕様は`swagger.yml`（OpenAPI 3.0形式）を参照してください。

## Swagger UIでの確認

```bash
# Swagger UIをDockerで起動
docker run -p 8080:8080 -e SWAGGER_JSON=/docs/swagger.yml -v $(pwd)/docs:/docs swaggerapi/swagger-ui
```

ブラウザで http://localhost:8080 を開く

## APIエンドポイント一覧

### 認証 (Auth)

| メソッド | パス | 説明 |
|----------|------|------|
| POST | /api/v1/auth/register | ユーザー登録 |
| POST | /api/v1/auth/verify-email | メールアドレス確認 |
| POST | /api/v1/auth/verify-email/resend | 確認メール再送信 |
| POST | /api/v1/auth/login | ログイン |
| POST | /api/v1/auth/logout | ログアウト |
| POST | /api/v1/auth/refresh | トークンリフレッシュ |
| POST | /api/v1/auth/password/forgot | パスワードリセット要求 |
| POST | /api/v1/auth/password/reset | パスワードリセット実行 |
| GET | /api/v1/auth/me | ユーザー情報取得 |
| PATCH | /api/v1/auth/me | ユーザー情報更新 |
| POST | /api/v1/auth/me/delete | アカウント削除 |

### 銃砲管理 (Firearms)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/firearms | 銃砲一覧取得 |
| POST | /api/v1/firearms | 銃砲登録 |
| GET | /api/v1/firearms/{id} | 銃砲詳細取得 |
| PATCH | /api/v1/firearms/{id} | 銃砲更新 |
| DELETE | /api/v1/firearms/{id} | 銃砲削除 |

### 実包種別 (AmmunitionTypes)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/ammunition-types | 実包種別一覧取得 |
| POST | /api/v1/ammunition-types | 実包種別登録 |
| GET | /api/v1/ammunition-types/{id} | 実包種別詳細取得 |
| PATCH | /api/v1/ammunition-types/{id} | 実包種別更新 |
| DELETE | /api/v1/ammunition-types/{id} | 実包種別削除 |

### 実包購入記録 (AmmunitionPurchases)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/ammunition-purchases | 購入記録一覧取得 |
| POST | /api/v1/ammunition-purchases | 購入記録登録 |
| GET | /api/v1/ammunition-purchases/{id} | 購入記録詳細取得 |
| PATCH | /api/v1/ammunition-purchases/{id} | 購入記録更新 |
| DELETE | /api/v1/ammunition-purchases/{id} | 購入記録削除 |

### 実包使用記録 (AmmunitionUsages)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/ammunition-usages | 使用記録一覧取得 |
| POST | /api/v1/ammunition-usages | 使用記録登録 |
| GET | /api/v1/ammunition-usages/{id} | 使用記録詳細取得 |
| PATCH | /api/v1/ammunition-usages/{id} | 使用記録更新 |
| DELETE | /api/v1/ammunition-usages/{id} | 使用記録削除 |

### 実包在庫 (AmmunitionStock)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/ammunition-stock | 在庫一覧取得 |
| GET | /api/v1/ammunition-stock/{ammunitionTypeId} | 在庫詳細取得 |

### 実包所持許可上限 (AmmunitionLimits)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/ammunition-limits | 上限一覧取得 |
| POST | /api/v1/ammunition-limits | 上限登録 |
| GET | /api/v1/ammunition-limits/{id} | 上限詳細取得 |
| PATCH | /api/v1/ammunition-limits/{id} | 上限更新 |
| DELETE | /api/v1/ammunition-limits/{id} | 上限削除 |

### 出猟記録 (HuntingRecords)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/hunting-records | 出猟記録一覧取得 |
| POST | /api/v1/hunting-records | 出猟記録登録 |
| GET | /api/v1/hunting-records/statistics | 出猟統計取得 |
| GET | /api/v1/hunting-records/{id} | 出猟記録詳細取得 |
| PATCH | /api/v1/hunting-records/{id} | 出猟記録更新 |
| DELETE | /api/v1/hunting-records/{id} | 出猟記録削除 |

### 帳簿出力 (Reports)

| メソッド | パス | 説明 |
|----------|------|------|
| GET | /api/v1/reports/ammunition-ledger | 実包管理帳簿PDF出力 |
| GET | /api/v1/reports/hunting-summary | 出猟サマリーPDF出力 |

## 認証

Bearer認証（JWT）を使用。

```
Authorization: Bearer <access_token>
```

### トークン有効期限

| トークン種別 | 有効期限 |
|--------------|----------|
| アクセストークン | 1時間 |
| リフレッシュトークン | 30日 |

## エラーレスポンス

```json
{
  "code": "ERROR_CODE",
  "message": "エラーメッセージ"
}
```

### 主なエラーコード

| コード | HTTPステータス | 説明 |
|--------|---------------|------|
| UNAUTHORIZED | 401 | 認証が必要 |
| INVALID_CREDENTIALS | 401 | 認証情報不正 |
| EMAIL_NOT_VERIFIED | 401 | メール未確認 |
| ACCOUNT_LOCKED | 423 | アカウントロック |
| VALIDATION_ERROR | 400 | バリデーションエラー |
| NOT_FOUND | 404 | リソース不存在 |
| INSUFFICIENT_STOCK | 422 | 在庫不足 |
| QUOTA_EXCEEDED | 422 | 許可上限超過 |

## 改訂履歴

| バージョン | 日付 | 内容 |
|------------|------|------|
| 1.0 | 2025-01-XX | 初版作成 |