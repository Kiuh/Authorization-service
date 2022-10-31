use bigdecimal::{BigDecimal, FromPrimitive};
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
    pub last_send_num: i64,
    pub setup_json: String,
    pub last_cell_num: i64,
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
                map_id, life_type, feed_type, setup_type, tick_period, 
                description, name, setup_json, last_send_num, last_cell_num
            FROM 
            (
                SELECT 
                    map_id, life_type, feed_type, setup_type, tick_period, description, 
                    name, usr.id, setup_json, last_send_num, last_cell_num
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
            last_send_num: res.last_send_num as i64,
            setup_json: res.setup_json,
            last_cell_num: res.last_cell_num as i64,
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

    pub async fn get_time<'a, E>(
        name: &str,
        login: &str,
        executor: E,
    ) -> crate::error::Result<BigDecimal>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let ticks_and_period = sqlx::query!(
            r#" 
                SELECT 
                    tick_period, last_send_num 
                FROM generations
                INNER JOIN users
                ON generations.owner_id = users.id
                WHERE generations.name = $1 AND users.login = $2 
            "#,
            name,
            login
        )
        .fetch_one(executor)
        .await
        .map_err(|e| ServerError::Database(e))?;

        Ok(ticks_and_period.tick_period
            * BigDecimal::from_i64(ticks_and_period.last_send_num).unwrap())
    }

    pub async fn update_last_send<'a, E>(
        name: &str,
        login: &str,
        send_num: i64,
        cell_num: i64,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#" 
                UPDATE 
                    generations
                SET 
                    last_send_num = GREATEST($1, last_send_num), 
                    last_cell_num = GREATEST($2, last_cell_num)
                WHERE 
                    name = $3 AND 
                    owner_id = (SELECT id FROM users WHERE login = $4)
            "#,
            send_num,
            cell_num,
            name,
            login,
        )
        .execute(executor)
        .await
        .map_err(|e| ServerError::Database(e))?;

        Ok(())
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
