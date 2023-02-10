use crate::error::ServerError;
use bigdecimal::BigDecimal;
use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub(super) struct GenWithIntellectId {
    pub intellect_id: i32,
    pub gen: Gen,
}

pub struct Gen {
    pub from_id: u64,
    pub to_id: u64,
    pub weight: BigDecimal,
}

impl GenWithIntellectId {
    pub async fn insert_many<'a, E>(
        gens: Vec<GenWithIntellectId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres>,
    {
        let (intellect_ids, from_ids, to_ids, weights): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = gens
            .into_iter()
            .map(|gen| {
                (
                    gen.intellect_id,
                    gen.gen.from_id as i32,
                    gen.gen.to_id as i32,
                    gen.gen.weight,
                )
            })
            .multiunzip();

        sqlx::query!(
            r#"
                INSERT INTO 
                gens(intellect_id, from_id, to_id, weight) 
                SELECT * FROM 
                UNNEST($1::INTEGER[], $2::INTEGER[], $3::INTEGER[], $4::DECIMAL[])
            "#,
            &intellect_ids,
            &from_ids,
            &to_ids,
            &weights
        )
        .execute(executor)
        .await
        .map_err(ServerError::Database)?;

        Ok(())
    }
}
