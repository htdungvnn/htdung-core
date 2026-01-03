mod auth;
mod config;
mod db;
mod error;
mod grpc;
mod state;

use sqlx::PgPool;
use tonic::transport::Server;

use config::Config;
use db::user_repo::UserRepo;
use grpc::auth_service::{auth::auth_service_server::AuthServiceServer, AuthGrpc};
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();

    let pool = PgPool::connect(&config.database_url).await?;
    println!("Connected to PostgreSQL");

    let state = AppState {
        users: UserRepo::new(pool),
        jwt_secret: config.jwt_secret,
    };

    let svc = AuthGrpc { state };

    Server::builder()
        .add_service(AuthServiceServer::new(svc))
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
