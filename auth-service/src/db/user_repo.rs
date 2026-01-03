use sqlx::PgPool;
use uuid::Uuid;
use sqlx::query;
use sqlx::query_as;


#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, email: &str, hash: &str) {
       query(
            "INSERT INTO users (id, email, password_hash, role)
            VALUES ($1, $2, $3, 'user')"
        )
        .bind(Uuid::new_v4())
        .bind(email)
        .bind(hash)
        .execute(&self.pool)
        .await?;
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Option<(Uuid, String, String)> {
        let row = query_as::<_, UserRow>(
            "SELECT id, password_hash, role FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
    }
}