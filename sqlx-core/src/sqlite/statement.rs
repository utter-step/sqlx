use core::ptr::{null_mut, NonNull};
use core::i32;

use std::str;
use std::ffi::CStr;
use std::convert::TryInto;
use std::collections::HashMap;

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use libsqlite3_sys::{
    SQLITE_INTEGER ,
            SQLITE_BLOB ,
            SQLITE_NULL ,
            SQLITE_TEXT ,
            SQLITE_FLOAT,
            sqlite3_column_int64,
    
    sqlite3_step, sqlite3_bind_parameter_count, sqlite3_column_decltype, sqlite3_column_type, sqlite3_column_name, sqlite3_column_count, SQLITE_DONE, SQLITE_ROW, sqlite3_stmt, sqlite3_finalize};

use crate::describe::{Describe, Column};
use crate::executor::Executor;
use crate::sqlite::value::SqliteValue;
use crate::sqlite::types::ValueKind;
use crate::sqlite::{Sqlite, SqliteTypeInfo, SqliteArguments, SqliteConnection, SqliteRow};

pub(super) enum Step {
    // indicates that an operation has completed
    Done,

    // indicates that another row of output is available
    Row,
}

pub(crate) struct Statement(pub(super) NonNull<sqlite3_stmt>);

// SAFE: See notes for the Send impl on [SqliteConnection].
#[allow(unsafe_code)]
unsafe impl Send for Statement {}

impl Statement {
    pub(super) fn step(&mut self) -> crate::Result<Step> {
        // <https://www.sqlite.org/c3ref/step.html>
        #[allow(unsafe_code)]
        let status = unsafe {
            sqlite3_step(self.0.as_ptr())
        };

        let step = match status {
            SQLITE_DONE => Step::Done,
            SQLITE_ROW => Step::Row,

            _ => {
                // TODO: Add handling of sqlite errors
                // We need to bubble up as a [DatabaseError]
                panic!("unexpected: {}", status);
            }
        };

        Ok(step)
    }

    // SAFETY: If called when we are not looking at a Row, will return Null
    pub(super) fn value(&mut self, i: usize) -> Option<SqliteValue> {
        match self.column_type(i) {
            ValueKind::Null => None,
            ValueKind::Int => Some(self.value_i64(i)),

            _ => todo!(),
        }
    }

    fn value_i64(&mut self, i: usize) -> SqliteValue {
        // <https://www.sqlite.org/c3ref/bind_parameter_count.html>
        #[allow(unsafe_code)]
        let val = unsafe {
            sqlite3_column_int64(self.0.as_ptr(), i as _)
        };

        SqliteValue::Int(val)
    }

    fn bind_parameter_count(&mut self) -> usize {
        // <https://www.sqlite.org/c3ref/bind_parameter_count.html>
        #[allow(unsafe_code)]
        let count = unsafe {
            sqlite3_bind_parameter_count(self.0.as_ptr())
        };

        count as usize
    }

    pub(super) fn describe(&mut self) -> crate::Result<Describe<Sqlite>> {
        let num_params = self.bind_parameter_count();
        
        // All bind params are null in sqlite
        let param_types = vec![SqliteTypeInfo::NULL; num_params].into_boxed_slice();
        
        let num_columns = self.column_count();

        let mut columns = Vec::with_capacity(num_columns);

        for i in 0..num_columns {
            let name = self.column_name(i);
            let type_ = self.column_decltype(i);

            columns.push(Column {
                name: Some(name),
                table_id: None,
                type_info: type_, 
            });
        }

        Ok(Describe {
            param_types,
            result_columns: columns.into_boxed_slice(),
        })
    }

    pub(super) fn column_decltype(&mut self, i: usize) -> SqliteTypeInfo {
        // [from_utf8_unchecked]: UTF-8 is guaranteed by the SQLite3 API
        // [sqlite3_column_decltype]: <https://www.sqlite.org/c3ref/column_decltype.html>
        #[allow(unsafe_code)]
        let name = unsafe {
            let name_ptr = sqlite3_column_decltype(self.0.as_ptr(), i as _);

            if name_ptr.is_null() {
                ""
            } else {
                str::from_utf8_unchecked(CStr::from_ptr(name_ptr).to_bytes())
            }
        };

        match name {
            "" => SqliteTypeInfo::NULL,
            "text" => SqliteTypeInfo::new(ValueKind::Text),
            "int" => SqliteTypeInfo::new(ValueKind::Int),
            "double" => SqliteTypeInfo::new(ValueKind::Double),
            "blob" => SqliteTypeInfo::new(ValueKind::Blob),

            // TODO: What are the possible return values here?
            _ => unreachable!("unexpected type name: {}", name)
        }
    }

    pub(super) fn column_name(&mut self, i: usize) -> Box<str> {
        // [from_utf8_unchecked]: UTF-8 is guaranteed by the SQLite3 API
        // [sqlite3_column_name]: <https://www.sqlite.org/c3ref/column_name.html>
        #[allow(unsafe_code)]
        let name = unsafe {
            str::from_utf8_unchecked(CStr::from_ptr(sqlite3_column_name(self.0.as_ptr(), i as _)).to_bytes())
        };

        name.into()
    }

    pub(super) fn column_names(&mut self) -> crate::Result<HashMap<Box<str>, usize>> {
        let count = self.column_count();
        let mut names = HashMap::with_capacity(count);

        for i in 0..count {
            names.insert(self.column_name(i), i);
        }

        Ok(names)
    }

    fn column_type(&mut self, i: usize) -> ValueKind {
        // <https://www.sqlite.org/c3ref/column_count.html>
        #[allow(unsafe_code)]
        let typ = unsafe {
            sqlite3_column_type(self.0.as_ptr(), i as _)
        };

        match typ {
            SQLITE_INTEGER => ValueKind::Int,
            SQLITE_BLOB => ValueKind::Blob,
            SQLITE_NULL => ValueKind::Null,
            SQLITE_TEXT => ValueKind::Text,
            SQLITE_FLOAT => ValueKind::Double,

            // TODO: What are the possible return values here?
            _ => unreachable!("unexpected column type: {}", typ)
        }
    }

    pub(super) fn column_count(&mut self) -> usize {
        // <https://www.sqlite.org/c3ref/column_count.html>
        #[allow(unsafe_code)]
        let count = unsafe {
            sqlite3_column_count(self.0.as_ptr())
        };

        count as usize
    }
    
    pub(super) fn finalize(&mut self) -> crate::Result<()> {
        // <https://www.sqlite.org/c3ref/finalize.html>
        #[allow(unsafe_code)]
        let status = unsafe {
            sqlite3_finalize(self.0.as_ptr())
        };

        Ok(())
    }
}
