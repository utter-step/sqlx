use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use crate::cursor::Cursor;
use crate::executor::{Execute, Executor};
use crate::postgres::protocol::{self, CommandComplete, Encode, Message, StatementId, TypeFormat};
use crate::postgres::{PgArguments, PgConnection, PgCursor, PgRow, PgTypeInfo, Postgres};
use crate::Error;
use futures_core::future::BoxFuture;
use tokio::macros::support::Future;

impl PgConnection {
    pub(crate) async fn send_execute(
        &mut self,
        query: &str,
        arguments: Option<PgArguments>,
    ) -> crate::Result<()> {
        if let Some(arguments) = arguments {
            // Check the statement cache for a statement ID that matches the given query
            // If it doesn't exist, we generate a new statement ID and write out [Parse] to the
            // connection command buffer
            let statement = self.write_prepare(query, &arguments);

            // Next, [Bind] attaches the arguments to the statement and creates a named portal
            self.write_bind("", statement, &arguments);

            // Next, [Describe] will return the expected result columns and types
            // Conditionally run [Describe] only if the results have not been cached
            // if !self.statement_cache.has_columns(statement) {
            //     self.write_describe(protocol::Describe::Portal(""));
            // }

            // Next, [Execute] then executes the unnamed portal
            self.write_execute("", 0);

            // Finally, [Sync] asks postgres to process the messages that we sent and respond with
            // a [ReadyForQuery] message when it's completely done. Theoretically, we could send
            // dozens of queries before a [Sync] and postgres can handle that. Execution on the server
            // is still serial but it would reduce round-trips. Some kind of builder pattern that is
            // termed batching might suit this.
            self.write_sync();
        } else {
            // https://www.postgresql.org/docs/12/protocol-flow.html#id-1.10.5.7.4
            self.write_simple_query(query);
        }

        self.wait_until_ready().await?;

        self.stream.flush().await?;
        self.is_ready = false;

        Ok(())
    }

    pub(crate) fn write_simple_query(&mut self, query: &str) {
        self.stream.write(protocol::Query(query));
    }

    pub(crate) fn write_prepare(&mut self, query: &str, args: &PgArguments) -> StatementId {
        // TODO: check query cache

        let id = StatementId(self.next_statement_id);

        self.next_statement_id += 1;

        self.stream.write(protocol::Parse {
            statement: id,
            query,
            param_types: &*args.types,
        });

        // TODO: write to query cache

        id
    }

    pub(crate) fn write_describe(&mut self, d: protocol::Describe) {
        self.stream.write(d);
    }

    pub(crate) fn write_bind(&mut self, portal: &str, statement: StatementId, args: &PgArguments) {
        self.stream.write(protocol::Bind {
            portal,
            statement,
            formats: &[TypeFormat::Binary],
            // TODO: Early error if there is more than i16
            values_len: args.types.len() as i16,
            values: &*args.values,
            result_formats: &[TypeFormat::Binary],
        });
    }

    pub(crate) fn write_execute(&mut self, portal: &str, limit: i32) {
        self.stream.write(protocol::Execute { portal, limit });
    }

    pub(crate) fn write_sync(&mut self) {
        self.stream.write(protocol::Sync);
    }

    pub(crate) async fn affected_rows(&mut self) -> crate::Result<u64> {
        let mut rows = 0;

        loop {
            match conn.stream.read().await? {
                Message::ParseComplete | Message::BindComplete => {
                    // ignore x_complete messages
                }

                Message::DataRow => {
                    // ignore rows
                    // TODO: should we log or something?
                }

                Message::CommandComplete => {
                    rows += CommandComplete::read(self.stream.buffer())?.affected_rows;
                }

                Message::ReadyForQuery => {
                    // done
                    self.is_ready = true;
                    break;
                }

                message => {
                    return Err(
                        protocol_err!("affected_rows: unexpected message: {:?}", message).into(),
                    );
                }
            }
        }

        Ok(rows)
    }
}

impl<'e> Executor<'e> for &'e mut super::PgConnection {
    type Database = Postgres;

    fn execute<'q, E>(&mut self, query: E) -> BoxFuture<crate::Result<u64>>
    where
        E: Execute<'q, Self::Database>,
    {
        let (query, arguments) = query.into_parts();
        Box::pin(async {
            self.send_execute(query, arguments).await?;
            self.affected_rows().await
        })
    }

    fn fetch<'q, E>(self, query: E) -> PgCursor<'e, 'q>
    where
        E: Execute<'q, Self::Database>,
    {
        PgCursor::from_connection(self, query)
    }

    #[doc(hidden)]
    #[inline]
    fn fetch_by_ref<'q, E>(&mut self, query: E) -> PgCursor<'_, 'q>
    where
        E: Execute<'q, Self::Database>,
    {
        self.fetch(query)
    }
}

impl_execute_for_query!(Postgres);
