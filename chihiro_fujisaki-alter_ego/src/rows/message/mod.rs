mod try_from_row;

pub struct Message
{
    id: u64,
    channel_id: u64,
    guild_id: Option<u64>
}
