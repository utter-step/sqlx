use crate::sqlite::statement::Statement;

pub struct SqliteRow2<'cur> {
    pub(super) statement: &'cur mut Statement,
}
