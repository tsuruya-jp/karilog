use presentation::handlers::auth_handlers::AppAuthState;
use shared::config::AppConfig;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .envファイルを読み込み
    dotenvy::dotenv().ok();

    // ロギング初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "karilog=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Karilog application...");

    // 設定読み込み
    let config = AppConfig::load()?;
    tracing::info!("Configuration loaded successfully");

    // DIコンテナ初期化
    let container = container::Container::new(config.clone()).await?;
    tracing::info!("DI container initialized successfully");

    // データベースマイグレーション実行
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&container.db_pool)
        .await?;
    tracing::info!("Database migrations completed successfully");

    // AuthStateを作成
    let auth_state = AppAuthState {
        auth_usecases: container.auth_usecases.clone(),
    };

    // Axumアプリ構築
    let app = presentation::routes::create_router(auth_state, container.jwt_service.clone())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // サーバー起動
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()?,
        config.server.port,
    ));

    tracing::info!("Server listening on {}", addr);
    tracing::info!("Environment: {}", config.server.env);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
