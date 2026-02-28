use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};

use super::env::Env;

pub struct Projection {
    pool: PgPool,
}

impl Projection {
    pub async fn connect(env: &Env) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env.database_url)
            .await
            .context("failed to connect to PostgreSQL")?;
        Ok(Self { pool })
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
        Ok(self.pool.begin().await?)
    }

    pub async fn query_one<T>(&self, sql: &str, params: &[&str]) -> Result<Option<T>>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let mut q = sqlx::query_as::<_, T>(sql);
        for p in params {
            q = q.bind(*p);
        }
        Ok(q.fetch_optional(&self.pool).await?)
    }

    pub async fn query_all<T>(&self, sql: &str) -> Result<Vec<T>>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        Ok(sqlx::query_as::<_, T>(sql).fetch_all(&self.pool).await?)
    }
}
