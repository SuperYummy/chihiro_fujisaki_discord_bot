use tokio_postgres::Row;
use try_into_message::try_into_message;

use crate::{aliases::error_result::Hope, rows::message::Message};

mod try_into_message;

pub trait Extension
{
    fn try_into_message(self) -> Hope<Message>;
}

impl Extension for Row
{
    fn try_into_message(self) -> Hope<Message>
    {
        try_into_message(self)
    }
}
