pub mod created;
pub mod module_changed;
pub mod removed;

#[derive(sqlx::Type)]
#[sqlx(type_name = "diff_type", rename_all = "snake_case")]
pub enum DiffType {
    CreateCell,
    ChangeModuleValue,
    RemoveCell,
}
