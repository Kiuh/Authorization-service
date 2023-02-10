use crate::error::ServerError;
use bigdecimal::BigDecimal;
use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub(super) struct NeuronWithIntellectId {
    pub intellect_id: i32,
    pub neuron: Neuron,
}

pub struct Neuron {
    pub bias: BigDecimal,
}

impl NeuronWithIntellectId {
    pub async fn insert_many<'a, E>(
        neurons: Vec<NeuronWithIntellectId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres>,
    {
        let (intellect_ids, biases): (Vec<_>, Vec<_>) = neurons
            .into_iter()
            .map(|neuron| (neuron.intellect_id, neuron.neuron.bias))
            .multiunzip();

        sqlx::query!(
            r#"INSERT INTO neurons(intellect_id, bias) SELECT * FROM UNNEST($1::INTEGER[], $2::DECIMAL[])"#,
            &intellect_ids,
            &biases,
        ).execute(executor).await.map_err(ServerError::Database)?;

        Ok(())
    }
}
