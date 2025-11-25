# Karilog - Claude Code開発ガイド

## プロジェクト概要

Karilogは射撃・狩猟の帳簿をデジタル管理するWebサービスです。
実包管理帳簿、出猟カレンダーなどの機能を提供し、警察提出用PDF出力に対応します。

## 技術スタック

- **バックエンド**: Rust (Axum)
- **フロントエンド**: React（後発）
- **データベース**: PostgreSQL
- **認証**: JWT (jsonwebtoken)
- **メール送信**: AWS SES
- **PDF生成**: genpdf

## アーキテクチャ

DDD（ドメイン駆動設計）+ Cargo Workspaceによるレイヤー分離

```
karilog/
├── domain/          # ドメイン層（エンティティ、値オブジェクト、リポジトリIF）
├── application/     # アプリケーション層（ユースケース、DTO）
├── infrastructure/  # インフラ層（リポジトリ実装、外部サービス）
├── presentation/    # プレゼンテーション層（Axumハンドラ）
├── shared/          # 共通モジュール（設定、エラー）
├── container/       # DIコンテナ
└── src/main.rs      # エントリポイント
```

### 依存関係の方向

```
presentation → application → domain ← infrastructure
                    ↓              ↓
                 shared ←──────────┘
                    ↑
               container
```

## 主要ドキュメント

- [要件定義書](docs/requirements.md)
- [DB設計書](docs/database.md)
- [API設計書](docs/api.md) - swagger.yml参照
- [アーキテクチャ設計書](docs/architecture.md)

## 開発ルール

### コーディング規約

1. **Rustフォーマット**: `cargo fmt`を使用
2. **Lint**: `cargo clippy`で警告なしを維持
3. **命名規則**:
   - ファイル名: snake_case
   - 構造体/列挙型: PascalCase
   - 関数/変数: snake_case
   - 定数: SCREAMING_SNAKE_CASE

### コミットメッセージ

```
<type>: <subject>

<body>
```

type:
- feat: 新機能
- fix: バグ修正
- docs: ドキュメント
- refactor: リファクタリング
- test: テスト
- chore: ビルド、CI等

### ブランチ戦略

- `main`: 本番リリース
- `develop`: 開発用
- `feature/*`: 機能開発
- `fix/*`: バグ修正

## 環境変数

```env
# サーバー設定
APP__SERVER__HOST=0.0.0.0
APP__SERVER__PORT=8080
APP__SERVER__ENV=development

# データベース
APP__DATABASE__URL=postgres://user:pass@localhost:5432/karilog
APP__DATABASE__MAX_CONNECTIONS=10
APP__DATABASE__MIN_CONNECTIONS=2

# JWT
APP__JWT__SECRET=your-secret-key
APP__JWT__ACCESS_TOKEN_EXPIRES_IN_SECS=3600
APP__JWT__REFRESH_TOKEN_EXPIRES_IN_DAYS=30

# メール (AWS SES)
APP__EMAIL__FROM_ADDRESS=noreply@karilog.jp
APP__EMAIL__FROM_NAME=Karilog
APP__EMAIL__VERIFICATION_URL_BASE=https://karilog.jp/verify
APP__EMAIL__PASSWORD_RESET_URL_BASE=https://karilog.jp/reset-password
```

## 開発コマンド

```bash
# ビルド
cargo build

# 開発サーバー起動
cargo run

# テスト実行
cargo test

# フォーマット
cargo fmt

# Lint
cargo clippy

# マイグレーション実行
sqlx migrate run

# マイグレーション作成
sqlx migrate add <name>
```

## 実装優先順位

### Phase 1: 認証基盤
1. ユーザー登録
2. メール確認
3. ログイン/ログアウト
4. トークンリフレッシュ
5. パスワードリセット
6. アカウントロック機能

### Phase 2: 銃砲・実包管理
1. 銃砲CRUD
2. 実包種別マスタCRUD
3. 実包所持許可上限CRUD
4. 実包購入記録CRUD
5. 実包使用記録CRUD
6. 在庫計算ロジック

### Phase 3: 出猟管理
1. 出猟記録CRUD
2. カレンダー表示用API
3. 統計API

### Phase 4: 帳簿出力
1. 実包管理帳簿PDF
2. 出猟サマリーPDF
3. CSV出力

## テスト方針

- **単体テスト**: domain層のビジネスロジック
- **統合テスト**: リポジトリ実装、API E2E
- **テストデータ**: fixtures/ディレクトリで管理

## 注意事項

1. **論理削除**: 全エンティティはsoft delete（deleted_atカラム）
2. **マルチテナンシー**: user_idで分離、必ずWHERE句に含める
3. **実包残数計算**: 購入合計 - 使用合計で算出
4. **アカウントロック**: 5回連続失敗で30分ロック
5. **トークン有効期限**: アクセス1時間、リフレッシュ30日