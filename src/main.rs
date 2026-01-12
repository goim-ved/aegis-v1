use axum::{
    routing::get,
    Router,
    extract::State,
};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use aegis_fintech_v1::state::AppState;
use aegis_fintech_v1::compliance;
use aegis_fintech_v1::finance;
use aegis_fintech_v1::governance;
use aegis_fintech_v1::agent;

use tower_http::cors::CorsLayer;

use tower::{ServiceBuilder, limit::RateLimitLayer};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!().run(&pool).await?;
    println!("Migrations applied successfully.");

    // Init Chain Client
    let rpc_url = env::var("RPC_URL").unwrap_or("http://127.0.0.1:8545".to_string());
    // Default key for hardhat node #0
    let private_key = env::var("PRIVATE_KEY").unwrap_or("".to_string()); // FORCE USER TO SET ENV VAR
    let contract_addr = env::var("CONTRACT_ADDRESS").unwrap_or("0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string());

    let chain_client = aegis_fintech_v1::chain::ChainClient::new(&rpc_url, &private_key, &contract_addr)
        .await
        .expect("Chain Client init failed. Is the Hardhat node running? (Check RPC_URL)");

    let state = AppState { 
        pool,
        chain: std::sync::Arc::new(chain_client),
    };

    // Middleware: Rate Limit (100 req/sec) & Strict CORS
    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:3000".parse().unwrap(), "http://127.0.0.1:3000".parse().unwrap()])
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

use axum::middleware;

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/compliance", compliance::router().route_layer(middleware::from_extractor::<aegis_fintech_v1::auth::Claims>()))
        .nest("/api/finance", finance::router().route_layer(middleware::from_extractor::<aegis_fintech_v1::auth::Claims>()))
        .nest("/api/governance", governance::router().route_layer(middleware::from_extractor::<aegis_fintech_v1::auth::Claims>()))
        .nest("/api/agent", agent::router().route_layer(middleware::from_extractor::<aegis_fintech_v1::auth::Claims>()))
        .nest("/api/auth", aegis_fintech_v1::auth::router())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(RateLimitLayer::new(100, Duration::from_secs(1)))
                .layer(cors)
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!(">> Aegis Core v0.1.0 active at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check(State(state): State<AppState>) -> String {
    // Check DB connection - critical for startup
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => "System: Online (DB Connected)".to_string(),
        Err(e) => format!("System: Degraded (DB Error: {})", e),
    }
}

