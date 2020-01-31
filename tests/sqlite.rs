use futures::TryStreamExt;
use sqlx::{Connect as _, Connection as _, Cursor as _, Executor as _, Row as _, SqliteConnection};

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn it_fetches_int() -> anyhow::Result<()> {
    let mut conn = connect().await?;

    // TODO: int -> float
    // TODO: ...

    // int -> int

    let mut cur = conn.fetch("SELECT 10", Default::default());
    let mut row = cur.try_next().await?.unwrap();
    let _1: i32 = row.get(0);

    assert_eq!(_1, 10);

    // NULL -> int

    let mut cur = conn.fetch("SELECT NULL", Default::default());
    let mut row = cur.try_next().await?.unwrap();
    let _1: i32 = row.get(0);

    assert_eq!(_1, 0);

    Ok(())
}

// #[cfg_attr(feature = "runtime-async-std", async_std::test)]
// #[cfg_attr(feature = "runtime-tokio", tokio::test)]
// async fn it_describes_a_system_table() -> anyhow::Result<()> {
//     let mut conn = connect().await?;

//     let desc = conn.describe("SELECT * FROM sqlite_master").await?;

//     assert_eq!(desc.param_types.len(), 0);

//     assert_eq!(desc.result_columns.len(), 5);
//     assert_eq!(desc.result_columns[2].name.as_deref(), Some("tbl_name"));

//     // conn.close().await?;

//     Ok(())
// }

async fn connect() -> anyhow::Result<SqliteConnection> {
    Ok(SqliteConnection::connect("sqlite://").await?)
}
