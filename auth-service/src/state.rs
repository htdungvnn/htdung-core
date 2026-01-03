use crate::db::user_repo::UserRepo;

#[derive(Clone)]
pub struct AppState {
    pub users: UserRepo,
    pub jwt_secret: String,
}
