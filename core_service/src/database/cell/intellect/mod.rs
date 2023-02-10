use itertools::{izip, MultiUnzip};
use sqlx::{Executor, Postgres};

use crate::error::ServerError;

pub mod gen;
pub mod neuron;

use gen::{Gen, GenWithIntellectId};
use neuron::{Neuron, NeuronWithIntellectId};

pub(super) struct IntellectWithCellId {
    pub intellect: Intellect,
    pub cell_id: i32,
}

pub struct Intellect {
    pub in_neuron_count: u32,
    pub out_neuron_count: u32,
    pub neurons: Vec<Neuron>,
    pub gens: Vec<Gen>,
}

impl IntellectWithCellId {
    pub async fn insert_many<'a, E>(
        intellects: Vec<IntellectWithCellId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        // Neurons and gens contain intellect_id NOT from database for now.
        let (cell_ids, in_neuron_counts, out_neuron_counts, neurons, gens): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = intellects
            .into_iter()
            .enumerate()
            .map(|(intellect_index, intellect)| {
                (
                    intellect.cell_id,
                    intellect.intellect.in_neuron_count as i32,
                    intellect.intellect.out_neuron_count as i32,
                    intellect
                        .intellect
                        .neurons
                        .into_iter()
                        .map(|neuron| NeuronWithIntellectId {
                            intellect_id: intellect_index as i32,
                            neuron,
                        })
                        .collect::<Vec<NeuronWithIntellectId>>(),
                    intellect
                        .intellect
                        .gens
                        .into_iter()
                        .map(|gen| GenWithIntellectId {
                            intellect_id: intellect_index as i32,
                            gen,
                        })
                        .collect::<Vec<GenWithIntellectId>>(),
                )
            })
            .multiunzip();

        let intellect_ids: Vec<_> = sqlx::query!(
            r#"
                INSERT INTO 
                intellect(cell_id, in_neuron_count, out_neuron_count) 
                SELECT * FROM UNNEST($1::INTEGER[], $2::INTEGER[], $3::INTEGER[]) 
                RETURNING id
            "#,
            &cell_ids,
            &in_neuron_counts,
            &out_neuron_counts
        )
        .fetch_all(executor.clone())
        .await
        .map_err(ServerError::Database)?
        .into_iter()
        .map(|res| res.id)
        .collect();

        // Rewrite intellect_ids in neurons and gens.
        let neurons: Vec<_> = neurons
            .into_iter()
            .flatten()
            .map(|neuron| NeuronWithIntellectId {
                intellect_id: intellect_ids[neuron.intellect_id as usize],
                neuron: neuron.neuron,
            })
            .collect();
        let gens: Vec<_> = gens
            .into_iter()
            .flatten()
            .map(|gen| GenWithIntellectId {
                intellect_id: intellect_ids[gen.intellect_id as usize],
                gen: gen.gen,
            })
            .collect();

        NeuronWithIntellectId::insert_many(neurons, executor.clone()).await?;
        GenWithIntellectId::insert_many(gens, executor).await?;

        Ok(())
    }
}

impl Intellect {
    pub async fn fetch<'a, E>(id: i32, executor: E) -> crate::error::Result<Intellect>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query!(
            r#"
                SELECT 
                    in_neuron_count, 
                    out_neuron_count,
                    (
                        SELECT ARRAY_AGG(bias::DECIMAL) FROM neurons WHERE intellect_id = $1
                    ) AS neuron_biases,
                    (
                        SELECT ARRAY_AGG(from_id::INTEGER) FROM gens WHERE intellect_id = $1
                    ) AS gen_from_ids,
                    (
                        SELECT ARRAY_AGG(to_id::INTEGER) FROM gens WHERE intellect_id = $1
                    ) AS gen_to_ids,
                    (
                        SELECT ARRAY_AGG(weight::DECIMAL) FROM gens WHERE intellect_id = $1
                    ) AS gen_weights
                FROM intellect
            "#,
            id
        )
        .fetch_one(executor)
        .await
        .map_err(ServerError::Database)
        .map(|res| Intellect {
            in_neuron_count: res.in_neuron_count as u32,
            out_neuron_count: res.out_neuron_count as u32,
            neurons: res
                .neuron_biases
                .unwrap()
                .into_iter()
                .map(|bias| Neuron { bias })
                .collect(),
            gens: izip!(
                res.gen_from_ids.unwrap(),
                res.gen_to_ids.unwrap(),
                res.gen_weights.unwrap()
            )
            .map(|(from_id, to_id, weight)| Gen {
                from_id: from_id as u64,
                to_id: to_id as u64,
                weight,
            })
            .collect(),
        })
    }
}
