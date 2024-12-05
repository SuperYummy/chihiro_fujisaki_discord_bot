use tokio_postgres::Row;

use crate::{aliases::error_result::Hope, rows::message::Message};

pub fn try_into_message(row: Row) -> Hope<Message>
{
    Message::try_from_row(row)
}
