use std::env;
use twilight_http::Client;

pub struct ChihiroFujisaki {
   pub bot: Client,
   pub token: String
}

impl ChihiroFujisaki {

    pub const TOKEN_KEY: &str = "DISCORD_TOKEN";


    pub fn summon() -> Self {

        let key_token = env::var(Self::TOKEN_KEY).unwrap();

        let cloned_token =  key_token.clone();
        

        Self {
            bot: Client::new(cloned_token),
            token: key_token
        }

    }

}
