use tokio_postgres::Client;

mod new;
mod select_message;

pub struct Database
{
    client: Client,
}
