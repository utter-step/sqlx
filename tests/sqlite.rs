use sqlx::{Connect as _, Connection as _, Executor as _, SqliteConnection, Row as _};

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn it_connects() -> anyhow::Result<()> {
    let mut conn = connect().await?;

    let row = sqlx::query("select 1 + ?")
        .bind(10)
        .fetch_one(&mut conn)
        .await?;

    // assert_eq!(2, row.get(0));

    // conn.close().await?;

    Ok(())
}

async fn connect() -> anyhow::Result<SqliteConnection> {
    Ok(SqliteConnection::connect("sqlite://").await?)
}
