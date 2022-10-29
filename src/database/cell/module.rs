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
    pub value: BigDecimal,
}

impl ModuleWithCellId {
    pub async fn insert_many<'a, E>(
        modules: Vec<ModuleWithCellId>,
        executor: E,
    ) -> crate::error::Result
    where
        E: Executor<'a, Database = Postgres>,
    {
        let (cell_ids, names, values): (Vec<_>, Vec<_>, Vec<_>) = modules
            .into_iter()
            .map(|module| (module.cell_id, module.module.name, module.module.value))
            .multiunzip();

        sqlx::query!(
            r#"INSERT INTO modules(cell_id, name, value) SELECT * FROM UNNEST($1::INTEGER[], $2::VARCHAR[], $3::DECIMAL[])"#,
            &cell_ids,
            &names,
            &values
        ).execute(executor).await.map_err(|e| ServerError::Database(e))?;

        Ok(())
    }
}
