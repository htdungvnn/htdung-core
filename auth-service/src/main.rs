use tonic::transport::Server;
use sqlx::PgPool;
use auth_service::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let pool = PgPool::connect(&config.database_url).await?;

    let state = AppState {
        users: UserRepo::new(pool),
        jwt_secret: config.jwt_secret,
    };

    let svc = AuthGrpc { state };

    Server::builder()
        .add_service(AuthServiceServer::new(svc))
        .serve("[::]:50051".parse()?)
        .await?;

    Ok(())
}