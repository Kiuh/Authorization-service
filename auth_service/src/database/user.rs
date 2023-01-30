use sqlx::{Executor, Postgres};

pub struct User {
    pub id: i32,
    pub data: UserData,
}

pub struct UserData {
    pub login: String,
    pub password: String,
    pub email: String,
}

impl UserData {
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
}

impl User {
    pub async fn get_id<'a, E>(email: &str, executor: E) -> crate::error::Result<Option<i32>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(
            sqlx::query!(r#"SELECT id FROM users WHERE email = $1"#, email)
                .fetch_optional(executor)
                .await?
                .map(|res| res.id),
        )
    }

    pub async fn get<'a, E>(login: &str, executor: E) -> crate::error::Result<Option<User>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"SELECT id, email, password FROM users WHERE login = $1"#,
            login
        )
        .fetch_optional(executor)
        .await?
        .map(|res| User {
            id: res.id,
            data: UserData {
                login: login.to_string(),
                email: res.email,
                password: res.password,
            },
        }))
    }

    pub async fn get_by_email<'a, E>(email: &str, executor: E) -> crate::error::Result<Option<User>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"SELECT id, login, password FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(executor)
        .await?
        .map(|res| User {
            id: res.id,
            data: UserData {
                login: res.login,
                email: email.to_string(),
                password: res.password,
            },
        }))
    }

    pub async fn update_auth_nonce<'a, E>(
        &self,
        nonce: i64,
        executor: E,
    ) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
                UPDATE users
                SET last_auth_nonce = $1
                WHERE login = $2 AND last_auth_nonce < $1
            "#,
            nonce,
            &self.data.login
        )
        .execute(executor)
        .await?
        .rows_affected()
            == 1)
    }
}
