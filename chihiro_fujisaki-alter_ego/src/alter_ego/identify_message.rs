use crate::{aliases::error_result::Hope, rows::message::Message};

use super::AlterEgo;

impl AlterEgo
{
    pub async fn identify_message(
        &self,
        message_id: &u64,
    ) -> Hope<Message>
    {
        self.database.select_message(message_id).await
    }
}
