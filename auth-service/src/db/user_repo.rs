use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, email: &str, hash: &str) {
        sqlx::query!(
            "INSERT INTO users (id, email, password_hash, role)
             VALUES ($1, $2, $3, 'user')",
            Uuid::new_v4(),
            email,
            hash
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Option<(Uuid, String, String)> {
        sqlx::query_as!(
            (Uuid, String, String),
            "SELECT id, password_hash, role FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }
}