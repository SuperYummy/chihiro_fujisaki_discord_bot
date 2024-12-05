use crate::{aliases::error_result::Hope, extensions::row::Extension, rows::message::Message};

use super::Database;

impl Database
{
    const MESSAGE_SELECTION: &str = "SELECT * FROM Message WHERE id = ?";

    pub async fn select_message(
        &self,
        message_id: &u64,
    ) -> Hope<Message>
    {
        let message_id_i64 = *message_id as i64;

        let message_row = self
            .client
            .query_one(Self::MESSAGE_SELECTION, &[&message_id_i64])
            .await?;

        message_row.try_into_message()
    }
}
