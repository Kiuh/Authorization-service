use sqlx::{Executor, Postgres};

pub struct User {
    pub login: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub async fn try_insert<'a, E>(&self, executor: E) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"INSERT INTO users (login, password, email) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"#,
            self.login,
            self.password,
            self.email
        )
        .execute(executor)
        .await?
        .rows_affected()
            == 1)
    }

    pub async fn is_exists<'a, E>(
        login: &str,
        password: &str,
        executor: E,
    ) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"SELECT * FROM users WHERE login = $1 AND password = $2"#,
            login,
            password
        )
        .fetch_optional(executor)
        .await?
        .is_some())
    }

    pub async fn get_id<'a, E>(login: &str, executor: E) -> crate::error::Result<Option<i64>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(
            sqlx::query!(r#"SELECT id FROM users WHERE login = $1"#, login)
                .fetch_optional(executor)
                .await?
                .map(|res| res.id as i64),
        )
    }
}
