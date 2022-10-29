use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub mod gen;
pub mod neuron;

use gen::Gen;
use neuron::Neuron;

use crate::error::ServerError;

use self::{gen::GenWithIntellectId, neuron::NeuronWithIntellectId};

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
                    intellect.cell_id as i32,
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
        .map_err(|e| ServerError::Database(e))?
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
