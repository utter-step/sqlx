use sqlx::{Connect as _, Connection as _, Executor as _, Row as _, SqliteConnection};

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn it_fetches_one() -> anyhow::Result<()> {
    let mut conn = connect().await?;

    let row = sqlx::query("SELECT 10").fetch_one(&mut conn).await?;

    // num values
    assert_eq!(row.len(), 1);

    let _1: i32 = row.get(0);

    assert_eq!(_1, 10);

    Ok(())
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn it_describes_a_system_table() -> anyhow::Result<()> {
    let mut conn = connect().await?;

    let desc = conn.describe("SELECT * FROM sqlite_master").await?;

    assert_eq!(desc.param_types.len(), 0);

    assert_eq!(desc.result_columns.len(), 5);
    assert_eq!(desc.result_columns[2].name.as_deref(), Some("tbl_name"));

    // conn.close().await?;

    Ok(())
}

async fn connect() -> anyhow::Result<SqliteConnection> {
    Ok(SqliteConnection::connect("sqlite://").await?)
}
