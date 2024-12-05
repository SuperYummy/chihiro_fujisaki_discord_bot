use crate::{aliases::error_result::Hope, database::Database};

use super::AlterEgo;

impl AlterEgo
{
    pub async fn summon(url: &str) -> Hope<Self>
    {
        let url_database = Database::new(url).await?;

        let alter_ego = Self { database: url_database };

        Ok(alter_ego)
    }
}
