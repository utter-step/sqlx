use core::marker::PhantomData;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

use std::future::Future;
use std::io;

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::ready;

use crate::cursor::Cursor;
use crate::database::HasRow;
use crate::pool::{Pool, PoolConnection};
use crate::postgres::protocol::{
    DataRow, Message, ParameterDescription, RowDescription, StatementId,
};
use crate::postgres::{PgConnection, PgRow};
use std::collections::HashMap;
use std::sync::Arc;

// IDEA 1 - Perhaps try using [`& &mut PgConnection`]

enum Step<'c> {
    Command(u64),
    NoData,
    Row(DataRow<'c>),
    ParamDesc(Box<ParameterDescription>),
    RowDesc(Box<RowDescription>),
}

enum Source<'a> {
    Empty,
    Connection(&'a mut PgConnection),
    PoolConnection(PoolConnection<PgConnection>),
    Pool(&'a Pool<PgConnection>),
}

impl Default for Source<'_> {
    fn default() -> Self {
        Source::Empty
    }
}

impl<'a> Source<'a> {
    async fn get(&'a mut self) -> crate::Result<&'a mut PgConnection> {
        Ok(match self {
            Source::Connection(conn) => conn,

            Source::PoolConnection(conn) => conn,

            Source::Pool(pool) => {
                *self = Source::PoolConnection(pool.acquire().await?);

                if let Source::PoolConnection(conn) = self {
                    conn
                } else {
                    // `self` was just set to a `PoolConnection(_)`
                    unreachable!()
                }
            }

            Source::Empty => {
                panic!("PgCursor must not be polled after it returns Poll::Ready");
            }
        })
    }
}

enum State<'a> {
    Ready,

    // If a cursor was directly `.await`-ed for the affected rows, we must store and poll
    // the future here
    WaitingForAffectedRows(BoxFuture<'a, crate::Result<u64>>),
}

impl Default for State<'_> {
    fn default() -> Self {
        State::Ready
    }
}

pub struct PgCursor<'con> {
    source: Source<'con>,
    statement: StatementId,
    state: State<'con>,
}

impl<'con> PgCursor<'con> {
    pub(crate) fn from_connection(statement: StatementId, conn: &'con mut PgConnection) -> Self {
        PgCursor {
            source: Source::Connection(conn),
            statement,
            state: State::Ready,
        }
    }
}

impl<'con> Cursor<'con> for PgCursor<'con> {
    type Database = super::Postgres;

    fn first(
        self,
    ) -> BoxFuture<'con, crate::Result<Option<<Self::Database as HasRow<'con>>::Row>>> {
        Box::pin(first(self.statement, self.source))
    }

    fn next<'cur: 'con>(
        &'cur mut self,
    ) -> BoxFuture<'cur, crate::Result<Option<<Self::Database as HasRow<'cur>>::Row>>> {
        //        loop {
        //            match self.state {
        //                State::InitialFromConnection(conn) => {
        //                    wait_until_ready(conn).await?;
        //                }
        //
        //                // Reached the end; continue to return [None] forever
        //                State::End => Ok(None),
        //
        //                _ => todo!(),
        //            }
        //        }
        todo!()
    }

    fn map<T, F>(self, f: F) -> BoxStream<'con, crate::Result<T>>
    where
        F: Fn(<Self::Database as HasRow<'con>>::Row) -> T,
    {
        todo!()
    }
}

// A [Cursor] can be awaited to receive the number of affected results
impl<'con> Future for PgCursor<'con> {
    type Output = crate::Result<u64>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut self.state {
                State::Ready => {
                    self.state = State::WaitingForAffectedRows(Box::pin(affected_rows(mem::take(
                        &mut self.source,
                    ))));
                }

                State::WaitingForAffectedRows(fut) => {
                    return fut.as_mut().poll(cx);
                }
            }
        }
    }
}

async fn affected_rows(mut src: Source<'_>) -> crate::Result<u64> {
    let conn = src.get().await?;

    wait_until_ready(conn).await?;
    conn.stream.flush().await?;
    conn.ready = false;

    let mut affected = 0;

    while let Some(step) = step(conn).await? {
        if let Step::Command(cnt) = step {
            affected += cnt;
        }
    }

    Ok(affected)
}

// noinspection ALL; IntellijRust incorrectly reports the lifetime here as needless
async fn first<'con>(
    statement: StatementId,
    mut src: Source<'con>,
) -> crate::Result<Option<PgRow<'con>>> {
    //    todo!()
    let conn = src.get().await?;

    wait_until_ready(conn).await?;
    conn.stream.flush().await?;
    conn.ready = false;

    let columns = get_columns(conn, statement).await?;

    while let Some(step) = step(conn).await? {
        if let Step::Row(row) = step {
            return Ok(Some(PgRow {
                data: row,
                columns: Arc::clone(&columns),
            }));
        }
    }

    Ok(None)
}

async fn step<'c>(conn: &'c mut PgConnection) -> crate::Result<Option<Step<'c>>> {
    match conn.receive().await? {
        //        match message {
        //            Some(Message::BindComplete)
        //            | Some(Message::ParseComplete) => {}
        Some(Message::CommandComplete(body)) => {
            return Ok(Some(Step::Command(body.affected_rows)));
        }

        Some(Message::NoData) => {
            return Ok(Some(Step::NoData));
        }

        Some(Message::DataRow(body)) => {
            return Ok(Some(Step::Row(body)));
        }

        Some(Message::ReadyForQuery(_)) => {
            //                conn.ready = true;

            return Ok(None);
        }

        Some(Message::ParameterDescription(desc)) => {
            return Ok(Some(Step::ParamDesc(desc)));
        }

        Some(Message::RowDescription(desc)) => {
            return Ok(Some(Step::RowDesc(desc)));
        }

        message => {
            return Err(protocol_err!("received unexpected message: {:?}", message).into());
        }
    }
}

//    // Connection was (unexpectedly) closed
//    Err(io::Error::from(io::ErrorKind::ConnectionAborted).into())
//}

async fn wait_until_ready(conn: &mut PgConnection) -> crate::Result<()> {
    if !conn.ready {
        while let Some(message) = conn.receive().await? {
            match message {
                Message::ReadyForQuery(_) => {
                    conn.ready = true;
                    break;
                }

                _ => {
                    // Drain the stream
                }
            }
        }
    }

    Ok(())
}

async fn get_columns(
    conn: &mut PgConnection,
    statement: StatementId,
) -> crate::Result<Arc<HashMap<Box<str>, usize>>> {
    if !conn.statement_cache.has_columns(statement) {
        let desc: Option<_> = 'outer: loop {
            while let Some(step) = step(conn).await? {
                match step {
                    Step::RowDesc(desc) => break 'outer Some(desc),

                    Step::NoData => break 'outer None,

                    _ => {}
                }
            }

            unreachable!();
        };

        let mut columns = HashMap::new();

        if let Some(desc) = desc {
            columns.reserve(desc.fields.len());

            for (index, field) in desc.fields.iter().enumerate() {
                if let Some(name) = &field.name {
                    columns.insert(name.clone(), index);
                }
            }
        }

        conn.statement_cache.put_columns(statement, columns);
    }

    Ok(conn.statement_cache.get_columns(statement))
}
