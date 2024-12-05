use aliases::error_result::Hope;
use chihiro_fujisaki::ChihiroFujisaki;
use dotenv::dotenv;
use events::on_event;
use twilight_gateway::{stream::create_recommended, Config, ConfigBuilder, Intents};
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{Activity, ActivityType, Status},
};

mod aliases;
mod chihiro_fujisaki;
mod events;

#[tokio::main]
async fn main() -> Hope<()>
{
    const TOKEN_EXPECTATION: &str = "The bot must have the token.";

    dotenv().ok();

    let chihiro_fujisaki = ChihiroFujisaki::summon().await?;

    let bot_token = chihiro_fujisaki
        .bot
        .token()
        .expect(TOKEN_EXPECTATION)
        .to_owned();

    let build_config = |_, builder: ConfigBuilder| -> Config {
        let main_state = "Hello World!";

        let owned_state = main_state.to_owned();

        let state_activity = Activity {
            application_id: None,
            assets: None,
            buttons: Vec::new(),
            created_at: None,
            details: None,
            emoji: None,
            flags: None,
            id: None,
            instance: None,
            kind: ActivityType::Custom,
            name: owned_state.to_owned(),
            party: None,
            secrets: None,
            state: Some(owned_state),
            timestamps: None,
            url: None,
        };

        let activity_playload = UpdatePresencePayload {
            activities: vec![state_activity],
            afk: false,
            since: None,
            status: Status::Online,
        };

        builder.presence(activity_playload).build()
    };

    let empty_intents = Intents::MESSAGE_CONTENT | Intents::GUILDS | Intents::GUILD_MESSAGES;

    let chihiro_config = Config::new(bot_token, empty_intents);

    let shards = create_recommended(&chihiro_fujisaki.bot, chihiro_config, build_config).await?;

    for mut shard in shards {
        while let event_error_result = shard.next_event().await {
            let event = match event_error_result {
                Ok(event) => event,
                Err(error) => {
                    println!("{}", error);

                    continue;
                }
            };

            on_event(&chihiro_fujisaki, event).await;
        }
    }

    Ok(())
}
