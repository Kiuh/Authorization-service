use bigdecimal::BigDecimal;
use sqlx::{Executor, Postgres};

use crate::error::ServerError;

#[derive(Default)]
pub struct Generation {
    pub name: String,
    pub map_id: String,
    pub life_type: String,
    pub feed_type: String,
    pub setup_type: String,
    pub tick_period: BigDecimal,
    pub last_send_num: u64, //
    pub setup_json: String, //
    pub last_cell_num: u64, //
    pub description: String,
}

impl Generation {
    pub async fn fetch_all<'a, E>(owner_id: &str, executor: E) -> crate::error::Result<Vec<Self>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
            SELECT 
            map_id, life_type, feed_type, setup_type, tick_period, description, name
            FROM 
            (
                SELECT map_id, life_type, feed_type, setup_type, tick_period, description, name, usr.id
                FROM
                generations
                INNER JOIN 
                (
                    SELECT id FROM users WHERE login = $1
                ) usr
                ON generations.owner_id = usr.id 
            ) gen_usr
            "#,
            owner_id
        )
        .fetch_all(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .into_iter()
        .map(|res| Generation {
            name: res.name,
            map_id: res.map_id,
            life_type: res.life_type,
            feed_type: res.feed_type,
            setup_type: res.setup_type,
            tick_period: res.tick_period,
            last_send_num: 0,
            setup_json: "".to_string(),
            last_cell_num: 0,
            description: res.description,
        })
        .collect())
    }

    pub async fn remove<'a, E>(name: &str, login: &str, executor: E) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"DELETE FROM generations WHERE name = $1 AND owner_id IN (SELECT id FROM users WHERE login = $2)"#,
            name,
            login
        )
        .execute(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .rows_affected()
            == 1)
    }

    pub async fn insert<'a, E>(&self, login: &str, executor: E) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"
                INSERT INTO generations
                (name, owner_id, map_id, life_type, feed_type, setup_type, tick_period, setup_json, description) 
                VALUES 
                ($1, (SELECT id FROM users WHERE login = $2), $3, $4, $5, $6, $7, $8, $9)
            "#,
            &self.name,
            login,
            &self.map_id,
            &self.life_type,
            &self.feed_type,
            &self.setup_type,
            &self.tick_period,
            &self.setup_json,
            &self.description
        )
        .execute(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .rows_affected()
            == 1)
    }
}

pub struct GenerationNameDescription {
    pub name: String,
    pub description: String,
}

impl GenerationNameDescription {
    pub async fn update<'a, E>(
        &self,
        old_name: &str,
        login: &str,
        executor: E,
    ) -> crate::error::Result<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(sqlx::query!(
            r#"UPDATE generations 
            SET name = $1, description = $2
            WHERE name = $3
            AND owner_id IN (SELECT id FROM users WHERE login = $4)"#,
            &self.name,
            &self.description,
            old_name,
            login
        )
        .execute(executor)
        .await
        .map_err(|e| ServerError::Database(e))?
        .rows_affected()
            == 1)
    }
}
