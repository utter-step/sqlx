use crate::postgres::protocol::{
    Authentication, BackendKeyData, CommandComplete, DataRow, NotificationResponse,
    ParameterDescription, ParameterStatus, ReadyForQuery, Response, RowDescription,
};
use std::fmt::{self, Debug};

#[repr(u8)]
pub enum Message<'c> {
    Authentication(Box<Authentication>),
    ParameterStatus(Box<ParameterStatus>),
    BackendKeyData(BackendKeyData),
    ReadyForQuery(ReadyForQuery),
    CommandComplete(CommandComplete),
    DataRow(DataRow<'c>),
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

impl<'c> Debug for Message<'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        f.write_str("TODO")
    }
}
