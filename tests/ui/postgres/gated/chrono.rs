fn main() {
    let _ = sqlx::query!("select now()::date");

    let _ = sqlx::query!("select now()::time");

    let _ = sqlx::query!("select now()::timestamp");

    let _ = sqlx::query!("select now()::timestamptz");
}
