use core::pin::Pin;
use core::task::{Context, Poll};

use std::collections::HashMap;
use std::future::Future;
use std::io::{self, ErrorKind::ConnectionAborted};
use std::mem::take;
use std::sync::Arc;

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

use crate::cursor::Cursor;
use crate::database::HasRow;
use crate::pool::{Pool, PoolConnection};
use crate::postgres::protocol::{
    DataRow, Message, ParameterDescription, RowDescription, StatementId,
};
use crate::postgres::{PgConnection, PgRow, Postgres};

pub(super) enum Source<'a> {
    Connection(&'a mut PgConnection),
    PoolConnection(PoolConnection<PgConnection>),
    Pool(&'a Pool<PgConnection>),
}

enum State<'a> {
    Ready,
    Row(DataRow<'a>),
    WaitingForAffectedRows(BoxFuture<'a, crate::Result<u64>>),
}

pub struct PgCursor<'a> {
    statement: StatementId,
    source: Source<'a>,
    state: State<'a>,
}

impl<'a> PgCursor<'a> {
    pub(crate) fn from_connection(
        connection: &'a mut PgConnection,
        statement: StatementId,
    ) -> Self {
        Self {
            statement,
            source: Source::Connection(connection),
            state: State::Ready,
        }
    }

    // TODO: [from_pool] probably needs to be the trait
    pub(crate) fn from_pool(pool: &'a Pool<PgConnection>, statement: StatementId) -> Self {
        Self {
            statement,
            source: Source::Pool(pool),
            state: State::Ready,
        }
    }
}

impl<'a> Source<'a> {
    async fn resolve(&mut self) -> crate::Result<&mut PgConnection> {
        if let Source::Pool(pool) = self {
            *self = Source::PoolConnection(pool.acquire().await?);
        }

        Ok(match self {
            Source::Connection(conn) => conn,
            Source::PoolConnection(conn) => conn,
            Source::Pool(_) => unreachable!(),
        })
    }
}

impl<'a> Cursor<'a> for PgCursor<'a> {
    type Database = Postgres;

    fn first(mut self) -> BoxFuture<'a, crate::Result<Option<PgRow<'a>>>> {
        // Box::pin(first(self))
        // return self.next();
        todo!()
    }

    fn next(&mut self) -> BoxFuture<crate::Result<Option<<Self::Database as HasRow>::Row>>> {
        Box::pin(next(self))
    }

    fn map<T, F>(self, f: F) -> BoxStream<'a, crate::Result<T>>
    where
        F: Fn(<Self::Database as HasRow<'a>>::Row) -> T,
    {
        todo!()
    }
}

impl<'a> Future for PgCursor<'a> {
    type Output = crate::Result<u64>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

async fn next<'e, 'e2: 'e>(cursor: &'e mut PgCursor<'e2>) -> crate::Result<Option<PgRow<'e>>> {
    let mut conn = cursor.source.resolve().await?;

    conn.stream.flush().await?;
    conn.ready = false;

    while let Some(message) = conn.receive().await? {
        match message {
            Message::DataRow(data) => {
                return Ok(Some(PgRow {
                    source: None,
                    data: Some(data),
                    columns: Arc::default(),
                }));
            }

            _ => {
                panic!("unexpected message: {:?}", message);
            }
        }
    }

    Ok(None)
}

async fn first<'a, 'a2: 'a>(mut cursor: PgCursor<'a2>) -> crate::Result<Option<PgRow<'a>>> {
    let mut conn = cursor.source.resolve().await?;

    conn.stream.flush().await?;
    conn.ready = false;

    while let Some(message) = conn.receive().await? {
        match message {
            Message::DataRow(data) => {
                // break;
                // cursor.state = State::Row(data);

                return Ok(Some(PgRow {
                    data: Some(data),
                    source: Some(cursor.source),
                    columns: Arc::default(),
                }));
            }

            _ => {
                panic!("unexpected message: {:?}", message);
            }
        }
    }

    // Ok(Some(PgRow {
    //     cursor: Some(cursor),
    //     data: None,
    //     columns: Arc::default(),
    // }))
    Ok(None)
}

async fn get_columns(
    conn: &mut PgConnection,
    statement: StatementId,
) -> crate::Result<Arc<HashMap<Box<str>, usize>>> {
    if !conn.statement_cache.has_columns(statement) {
        let description = match conn.receive().await? {
            Some(Message::RowDescription(rd)) => Some(rd),
            Some(Message::NoData) => None,

            Some(message) => {
                panic!("unexpected message: {:?}", message);
            }

            None => {
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted).into());
            }
        };

        let mut columns = HashMap::new();

        if let Some(description) = description {
            columns.reserve(description.fields.len());

            for (index, field) in description.fields.iter().enumerate() {
                if let Some(name) = &field.name {
                    columns.insert(name.clone(), index);
                }
            }
        }

        conn.statement_cache.put_columns(statement, columns);
    }

    Ok(conn.statement_cache.get_columns(statement))
}
