use tonic::{Request, Response, Status};

use crate::{
    auth::{jwt, password},
    error::AuthError,
    state::AppState,
};

pub mod auth {
    tonic::include_proto!("auth");
}
/* ===== SERVICE ===== */
pub struct AuthGrpc {
    pub state: AppState,
}

#[tonic::async_trait]
impl auth::auth_service_server::AuthService for AuthGrpc {
    async fn register(
        &self,
        req: Request<auth::RegisterRequest>,
    ) -> Result<Response<auth::AuthResponse>, Status> {
        let r = req.into_inner();
        let hash = password::hash(&r.password);

        self.state
            .users
            .create(&r.email, &hash)
            .await
            .map_err(AuthError::from)?;

        Ok(Response::new(auth::AuthResponse {
            token: "".to_string(),
        }))
    }

    async fn login(
        &self,
        req: Request<auth::LoginRequest>,
    ) -> Result<Response<auth::AuthResponse>, Status> {
        let r = req.into_inner();

        let (id, hash, role) = self
            .state
            .users
            .find_by_email(&r.email)
            .await
            .map_err(AuthError::from)?
            .ok_or(AuthError::InvalidCredentials)?;

        if !password::verify(&r.password, &hash) {
            return Err(AuthError::InvalidCredentials.into());
        }

        let token = jwt::create(&id.to_string(), &role, &self.state.jwt_secret);

        Ok(Response::new(auth::AuthResponse { token }))
    }

    async fn validate_token(
        &self,
        req: Request<auth::ValidateTokenRequest>,
    ) -> Result<Response<auth::ValidateTokenResponse>, Status> {
        let claims = jwt::decode(&req.into_inner().token, &self.state.jwt_secret)
            .map_err(AuthError::from)?;

        Ok(Response::new(auth::ValidateTokenResponse {
            valid: true,
            user_id: claims.sub,
            role: claims.role,
        }))
    }
}
