#[derive(Debug)]
pub enum SqliteValue {
    // TODO: Take by reference to remove the allocation
    Text(String),

    // TODO: Take by reference to remove the allocation
    Blob(Vec<u8>),

    Double(f64),

    Int(i64),

    Null,
}
