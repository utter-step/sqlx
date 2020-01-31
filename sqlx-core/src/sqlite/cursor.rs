use std::marker::PhantomData;

use futures_core::future::BoxFuture;
use futures_util::future::ready;

use crate::cursor::Cursor;
use crate::database::{HasCursor, HasRawRow};
use crate::sqlite::row::SqliteRow;
use crate::sqlite::statement::{Statement, Step};
use crate::sqlite::Sqlite;

enum CursorState {
    Initial(crate::Result<Statement>),
    Step(Statement),
}

pub struct SqliteCursor<'con> {
    state: Option<CursorState>,

    // TODO: Instead of owning this object, it would be better to just have a ptr to it
    pin: PhantomData<&'con Statement>,
}

impl<'con> SqliteCursor<'con> {
    pub(crate) fn new(statement: crate::Result<Statement>) -> Self {
        Self {
            state: Some(CursorState::Initial(statement)),
            pin: PhantomData,
        }
    }
}

impl<'con> Cursor<'con> for SqliteCursor<'con> {
    type Database = Sqlite;

    // TODO: This was brute force but it works, think through this more
    fn try_next<'cur>(
        &'cur mut self,
    ) -> BoxFuture<'cur, crate::Result<Option<<Self::Database as HasRawRow<'cur>>::RawRow>>> {
        match self.state.take() {
            None => {
                return Box::pin(ready(Ok(None)));
            }

            Some(CursorState::Initial(Err(error))) => {
                return Box::pin(ready(Err(error)));
            }

            Some(CursorState::Initial(Ok(statement))) => {
                self.state = Some(CursorState::Step(statement));
            }

            Some(CursorState::Step(statement)) => {
                self.state = Some(CursorState::Step(statement));
            }
        }

        if let Some(CursorState::Step(ref mut statement)) = self.state {
            return Box::pin(match statement.step() {
                Ok(Step::Row) => ready(Ok(Some(SqliteRow { statement }))),

                Ok(Step::Done) => {
                    // self.state = CursorState::End;
                    ready(Ok(None))
                }

                Err(error) => ready(Err(error)),
            });
        }

        Box::pin(ready(Ok(None)))
    }
}

impl<'a> HasCursor<'a> for Sqlite {
    type Database = Sqlite;

    type Cursor = SqliteCursor<'a>;
}
