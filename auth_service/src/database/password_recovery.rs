use sqlx::{Executor, Postgres, Transaction};

pub async fn try_update_nonce<'a, E>(
    user_id: i32,
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
            WHERE id = $2 AND last_recover_password_nonce < $1
        "#,
        nonce,
        user_id
    )
    .execute(executor)
    .await?
    .rows_affected()
        == 1)
}

pub async fn add<'a, E>(user_id: i32, access_code: &str, executor: E) -> crate::error::Result<()>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        r#"
            INSERT INTO password_recovery_requests 
                (user_id, access_code) 
            VALUES
                ($1, $2)
            ON CONFLICT(user_id) DO UPDATE
                SET 
                    access_code = EXCLUDED.access_code;
        "#,
        user_id,
        access_code
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_access_code<'a, E>(
    user_id: i32,
    executor: E,
) -> crate::error::Result<Option<String>>
where
    E: Executor<'a, Database = Postgres>,
{
    Ok(sqlx::query!(
        r#"
            SELECT access_code FROM password_recovery_requests WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(executor)
    .await?
    .access_code)
}

pub async fn apply(
    user_id: i32,
    new_password: &str,
    tx: &mut Transaction<'_, Postgres>,
) -> crate::error::Result {
    tx.execute(sqlx::query!(
        r#"
            UPDATE users SET password = $1 WHERE id = $2;
        "#,
        new_password,
        user_id
    ))
    .await?;

    tx.execute(sqlx::query!(
        r#"
            DELETE FROM password_recovery_requests
            WHERE user_id = $1;
        "#,
        user_id
    ))
    .await?;

    Ok(())
}
