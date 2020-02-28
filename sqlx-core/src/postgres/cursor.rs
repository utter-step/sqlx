use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use async_stream::try_stream;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

use crate::connection::{ConnectionSource, MaybeOwnedConnection};
use crate::cursor::Cursor;
use crate::database::HasRow;
use crate::executor::Execute;
use crate::pool::{Pool, PoolConnection};
use crate::postgres::cursor::State::Yielded;
use crate::postgres::protocol::{CommandComplete, DataRow, Message, StatementId};
use crate::postgres::{PgArguments, PgConnection, PgRow};
use crate::{Database, Postgres};
use futures_core::Stream;

pub struct PgCursor<'c, 'q> {
    conn: &'c mut PgConnection,
    query_args: Option<(&'q str, Option<PgArguments>)>,
}

impl<'c, 'q> PgCursor<'c, 'q> {
    #[doc(hidden)]
    pub(crate) fn from_connection<E>(conn: &'c mut PgConnection, query: E) -> Self
    where
        Self: Sized,
        E: Execute<'q, Postgres>,
    {
        let (query, arguments) = query.into_parts();

        Self {
            conn,
            query_args: Some((query, arguments)),
        }
    }

    // Optimization: shadows the trait method and avoids the overhead of boxing
    #[doc(hidden)]
    pub async fn next(&mut self) -> crate::Result<Option<PgRow>> {
        if let Some((query, arguments)) = self.query_args.take() {
            self.conn.send_execute(query, arguments).await?;
        }

        loop {
            match self.conn.stream.read().await? {
                Message::ParseComplete | Message::BindComplete => {
                    // ignore x_complete messages
                }

                Message::CommandComplete => {
                    // no more rows
                    break;
                }

                Message::DataRow => {
                    let data = DataRow::read(&mut self.conn)?;

                    return Ok(Some(PgRow {
                        connection: &mut self.conn,
                        columns: Arc::default(),
                        data,
                    }));
                }

                message => {
                    return Err(protocol_err!("next: unexpected message: {:?}", message).into());
                }
            }
        }

        Ok(None)
    }
}

impl<'c, 'q> Cursor<'c, 'q, Postgres> for PgCursor<'c, 'q> {
    fn next(&mut self) -> BoxFuture<crate::Result<Option<PgRow>>> {
        Box::pin(self.next())
    }
}
