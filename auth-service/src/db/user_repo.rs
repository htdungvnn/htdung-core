use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

/* ===== ROW TYPE ===== */
#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    password_hash: String,
    role: String,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, email: &str, hash: &str) -> Result<(), sqlx::Error> {
        query(
            "INSERT INTO users (id, email, password_hash, role)
             VALUES ($1, $2, $3, 'user')",
        )
        .bind(Uuid::new_v4())
        .bind(email)
        .bind(hash)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(Uuid, String, String)>, sqlx::Error> {
        let row =
            query_as::<_, UserRow>("SELECT id, password_hash, role FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(|u| (u.id, u.password_hash, u.role)))
    }
}
