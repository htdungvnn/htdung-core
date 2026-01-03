use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Internal error")]
    Internal,
}

impl From<AuthError> for Status {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => {
                Status::unauthenticated("Invalid credentials")
            }
            AuthError::Unauthorized => {
                Status::unauthenticated("Unauthorized")
            }
            AuthError::Database(_) => {
                Status::internal("Database error")
            }
            AuthError::Jwt(_) => {
                Status::unauthenticated("Invalid token")
            }
            AuthError::Internal => {
                Status::internal("Internal server error")
            }
        }
    }
}