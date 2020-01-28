use std::sync::Arc;
use std::collections::HashMap;

use crate::sqlite::SqliteConnection;
use crate::sqlite::SqliteRow2;
use crate::sqlite::statement::Statement;

pub struct SqliteCursor<'con> {
    pub(super) connection: &'con mut SqliteConnection,
    pub(super) statement: crate::Result<Option<Statement>>,
    pub(super) columns: Option<Arc<HashMap<Box<str>, usize>>>
}

impl<'con> SqliteCursor<'con> {
    pub async fn next<'cur>(&'cur mut self) -> crate::Result<Option<SqliteRow2<'cur>>> {



        match &mut self.statement {
            Ok(statement) => {
                Ok(Some(SqliteRow2 {     
                    statement
                 }))
            }

            // ?
            Err(e) => Err(e.clone())
        }

               //     while let Step::Row = statement.step()? {
        //         let mut values = Vec::with_capacity(columns.len());

        //         for i in 0..columns.len() {
        //             values.push(statement.value(i));
        //         }

        //         yield SqliteRow2 {
        //             // values: values.into_boxed_slice(),
        //             // columns: Arc::clone(&columns),
        //             statement: statement.clone()
        //         }
        //     }
    }
}
