use twilight_model::gateway::payload::incoming::MessageCreate as Message;
use crate::{aliases::error_result::Hope, chihiro_fujisaki::ChihiroFujisaki};

const USER_GREETING: &str = "hi chihiro";


pub async fn on_message_create(
    chihiro_fujisaki: &ChihiroFujisaki,
    message: Message
) -> Hope<()>
{

    println!("On the message {} creation, the user {} said {}", message.id, message.author.id, message.content);
    

    if message.content.to_lowercase().starts_with(USER_GREETING) {
        
        let chihiro_greeting =   format!( "Hi {}", message.author.name);

        chihiro_fujisaki.bot
        .create_message(message.channel_id)
        .content(&chihiro_greeting)?
        .await?;

    }


    Ok(())

}