use crate::error::ServerError;
use bigdecimal::BigDecimal;
use itertools::MultiUnzip;
use sqlx::{Executor, Postgres};

pub(super) struct ModuleWithCellId {
    pub cell_id: i32,
    pub module: Module,
}

pub struct Module {
    pub name: String,
    pub value: Option<BigDecimal>,
}

impl ModuleWithCellId {
    pub async fn insert_many<'a, E>(
        modules: Vec<ModuleWithCellId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres> + Clone,
    {
        let (modules_not_null, modules_null): (Vec<_>, Vec<_>) = modules
            .into_iter()
            .partition(|module| module.module.value.is_some());

        Self::insert_many_not_null(modules_not_null, executor.clone()).await?;
        Self::insert_many_null(modules_null, executor).await?;

        Ok(())
    }

    async fn insert_many_null<'a, E>(
        modules: Vec<ModuleWithCellId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres>,
    {
        let (cell_ids, names): (Vec<_>, Vec<_>) = modules
            .into_iter()
            .map(|module| (module.cell_id, module.module.name))
            .multiunzip();

        sqlx::query!(
            r#"INSERT INTO modules(cell_id, name) SELECT * FROM UNNEST($1::INTEGER[], $2::VARCHAR[])"#,
            &cell_ids,
            &names
        ).execute(executor).await.map_err(ServerError::Database)?;

        Ok(())
    }

    async fn insert_many_not_null<'a, E>(
        modules: Vec<ModuleWithCellId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres>,
    {
        let (cell_ids, names, values): (Vec<_>, Vec<_>, Vec<_>) = modules
            .into_iter()
            .map(|module| {
                (
                    module.cell_id,
                    module.module.name,
                    module.module.value.unwrap(),
                )
            })
            .multiunzip();

        sqlx::query!(
            r#"INSERT INTO modules(cell_id, name, value) SELECT * FROM UNNEST($1::INTEGER[], $2::VARCHAR[], $3::DECIMAL[])"#,
            &cell_ids,
            &names,
            &values
        ).execute(executor).await.map_err(ServerError::Database)?;

        Ok(())
    }
}
