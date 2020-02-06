use std::fmt::{self, Debug};

use crate::postgres::protocol::{
    Authentication, BackendKeyData, CommandComplete, DataRow, NotificationResponse,
    ParameterDescription, ParameterStatus, ReadyForQuery, Response, RowDescription,
};

// 's: the lifetime of the database server connection or socket
#[repr(u8)]
pub enum Message<'s> {
    Authentication(Box<Authentication>),
    ParameterStatus(Box<ParameterStatus>),
    BackendKeyData(BackendKeyData),
    ReadyForQuery(ReadyForQuery),
    CommandComplete(CommandComplete),
    DataRow(DataRow<'s>),
    Response(Box<Response>),
    NotificationResponse(Box<NotificationResponse>),
    ParseComplete,
    BindComplete,
    CloseComplete,
    NoData,
    PortalSuspended,
    ParameterDescription(Box<ParameterDescription>),
    RowDescription(Box<RowDescription>),
}

impl Debug for Message<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Message::*;

        f.write_str(match self {
            Authentication(_) => "Authentication",
            ParameterStatus(_) => "ParameterStatus",
            BackendKeyData(_) => "BackendKeyData",
            ReadyForQuery(_) => "ReadyForQuery",
            CommandComplete(_) => "CommandComplete",
            DataRow(_) => "DataRow",
            Response(_) => "Response",
            NotificationResponse(_) => "NotificationResponse",
            ParseComplete => "ParseComplete",
            BindComplete => "BindComplete",
            CloseComplete => "CloseComplete",
            NoData => "NoData",
            PortalSuspended => "PortalSuspended",
            ParameterDescription(_) => "ParameterDescription",
            RowDescription(_) => "RowDescription",
        })
    }
}
