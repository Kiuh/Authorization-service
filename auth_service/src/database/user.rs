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

    pub async fn get_password<'a, E>(
        login: &str,
        executor: E,
    ) -> crate::error::Result<Option<String>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(
            sqlx::query!(r#"SELECT password FROM users WHERE login = $1"#, login)
                .fetch_optional(executor)
                .await?
                .map(|res| res.password),
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
            login: login.to_string(),
            email: res.email,
            password: res.password,
        }))
    }

    pub async fn update_recover_password_nonce<'a, E>(
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
                SET last_recover_password_nonce = $1
                WHERE login = $2 AND last_recover_password_nonce < $1
            "#,
            nonce,
            &self.login
        )
        .execute(executor)
        .await?
        .rows_affected()
            == 1)
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
            &self.login
        )
        .execute(executor)
        .await?
        .rows_affected()
            == 1)
    }

    pub async fn request_new_password<'a, E>(
        &self,
        new_password: &str,
        access_code: &str,
        executor: E,
    ) -> crate::error::Result<()>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
                UPDATE users
                SET 
                    new_password = $1,
                    recover_password_access_code = $2
                WHERE login = $3
            "#,
            new_password,
            access_code,
            &self.login
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn get_recover_password_access_code<'a, E>(
        login: &str,
        executor: E,
    ) -> crate::error::Result<Option<String>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
                SELECT recover_password_access_code FROM users WHERE login = $1
            "#,
            login
        )
        .fetch_one(executor)
        .await?
        .recover_password_access_code)
    }

    pub async fn set_new_password<'a, E>(login: &str, executor: E) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
                UPDATE users 
                SET 
                    password = new_password, 
                    new_password = NULL,
                    recover_password_access_code = NULL
                WHERE login = $1
            "#,
            login
        )
        .execute(executor)
        .await?
        .rows_affected()
            == 1)
    }
}
