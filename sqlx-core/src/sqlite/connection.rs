use core::ptr::{null, null_mut, NonNull};

use std::convert::TryInto;
use std::ffi::CString;

use async_std::task::spawn_blocking;
use futures_core::future::BoxFuture;
use libsqlite3_sys::{
    sqlite3, sqlite3_open_v2, SQLITE_OK, SQLITE_OPEN_CREATE, SQLITE_OPEN_NOMUTEX,
    SQLITE_OPEN_READWRITE, SQLITE_OPEN_SHAREDCACHE,
};

use crate::connection::{Connect, Connection};
use crate::url::Url;

pub struct SqliteConnection {
    handle: NonNull<sqlite3>,
}

// SAFE: A sqlite3 handle is safe to access from multiple threads provided
//       that only one thread access it at a time. Or in other words,
//       the same guarantees that [Sync] requires. This is upheld as long
//       [SQLITE_CONFIG_MULTITHREAD] is enabled and [SQLITE_THREADSAFE] was
//       enabled when sqlite was compiled. We refuse to work if these conditions are
//       not upheld, see [SqliteConnection::establish].
//
// <https://www.sqlite.org/c3ref/threadsafe.html>
// <https://www.sqlite.org/c3ref/c_config_covering_index_scan.html#sqliteconfigmultithread>
#[allow(unsafe_code)]
unsafe impl Send for SqliteConnection {}

impl SqliteConnection {
    fn establish(url: crate::Result<Url>) -> crate::Result<Self> {
        let url = url?;

        // By default, we connect to an in-memory database.
        // TODO: Handle the error when there are internal NULs in the database URL
        let filename = CString::new(url.database().unwrap_or(":memory:")).unwrap();
        let mut handle: *mut sqlite3 = null_mut();

        // [SQLITE_OPEN_NOMUTEX] will instruct [sqlite3_open_v2] to return an error if it
        // cannot satisfy our wish for a thread-safe, lock-free connection object
        // TODO: Expose configuration for these
        let flags = SQLITE_OPEN_READWRITE
            | SQLITE_OPEN_CREATE
            | SQLITE_OPEN_NOMUTEX
            | SQLITE_OPEN_SHAREDCACHE;

        // SAFE: [filename] and [handle] must point to valid memory
        //
        // <https://www.sqlite.org/c3ref/open.html>
        #[allow(unsafe_code)]
        let status = unsafe {
            // TODO: This probably blocks IO
            sqlite3_open_v2(filename.as_ptr(), &mut handle, flags, null())
        };

        if status != SQLITE_OK {
            // TODO: Add handling of sqlite errors
            // We need to bubble up as a [DatabaseError]
            panic!("not ok: {}", status);
        }

        Ok(Self {
            // TODO: Handle the error when NULL is returned from [sqlite3_open_v2]
            handle: NonNull::new(handle).unwrap(),
        })
    }
}

impl Connect for SqliteConnection {
    type Connection = SqliteConnection;

    fn connect<T>(url: T) -> BoxFuture<'static, crate::Result<SqliteConnection>>
    where
        T: TryInto<Url, Error = crate::Error>,
        Self: Sized,
    {
        let url = url.try_into();

        // TODO: Establishing the connection obviously is blocking IO.. right?
        Box::pin(spawn_blocking(move || Self::establish(url)))
    }
}

impl Connection for SqliteConnection {
    fn close(self) -> BoxFuture<'static, crate::Result<()>> {
        todo!()
    }
}
