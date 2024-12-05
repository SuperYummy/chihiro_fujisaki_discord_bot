use super::Message;
use crate::aliases::error_result::Hope;
use tokio_postgres::Row;

impl Message
{
    pub fn try_from_row(row: Row) -> Hope<Message>
    {
        const ID_COLUMN: &str = "id";

        const CHANNEL_ID_COLUMN: &str = "channel_id";

        const GUILD_ID_COLUMN: &str = "guild_id";

        let column_guild_id_option = row.get::<_, Option<i64>>(GUILD_ID_COLUMN);

        let into_u64 = |guild_id: i64| -> u64 { guild_id as u64 };

        let row_message = Self {
            id: row.get::<_, i64>(ID_COLUMN) as u64,
            channel_id: row.get::<_, i64>(CHANNEL_ID_COLUMN) as u64,
            guild_id: column_guild_id_option.map(into_u64),
        };

        Ok(row_message)
    }
}
