use sqlx::{Connect, Cursor, Executor, PgConnection, PgPool};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // let mut conn = PgConnection::connect("postgres://postgres@localhost").await?;
    let pool = PgPool::new("postgres://postgres@localhost").await?;

    let mut cursor = pool.execute("SELECT 1");

    while let Some(row) = cursor.next().await? {
        println!(" -> 1/2/3");
    }

    Ok(())
}
