use std::env;

use chihiro_fujisaki_alter_ego::alter_ego::AlterEgo;
use twilight_http::Client;

use crate::aliases::error_result::Hope;

use super::ChihiroFujisaki;

impl ChihiroFujisaki
{
    pub async fn summon() -> Hope<Self>
    {
        const DATABASE_URL: &str = "DATABASE_URL";

        const TOKEN_KEY: &str = "DISCORD_TOKEN";

        let token_expectation = format!("The file .env must have the key {TOKEN_KEY}");

        let key_token = env::var(TOKEN_KEY).expect(&token_expectation);

        let chihiro_fujisaki = Self {
            bot: Client::new(key_token),
            alter_ego: AlterEgo::summon(DATABASE_URL).await?,
        };

        Ok(chihiro_fujisaki)
    }
}
