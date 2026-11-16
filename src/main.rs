use crate::core::{config::Config,database};
use sqlx::PgPool;
mod core;
mod models;
mod handlers;
mod services;
mod routers;
mod logging;

#[derive(Clone)]
struct AppState{
    name : String,
    config : Config,
    pgpool: PgPool
}

#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();
    // get config
    let config=Config::from_env().await.expect("Failed to load config");
    // create pool
    let pool=database::create_pool(&config.database_url).await.expect("Failed to connect to database!");
    // create state
    let state = AppState {
        name : "tasker".to_string(),
        config : config,
        pgpool : pool
    };
    // create router
    let app = routers::task::create_router()
    .await
    .expect("Error creating Router")
    .with_state(state);
    


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}