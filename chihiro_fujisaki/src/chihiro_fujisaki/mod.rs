use chihiro_fujisaki_alter_ego::alter_ego::AlterEgo;
use twilight_http::Client;

mod summon;

pub struct ChihiroFujisaki {
   pub bot: Client,
   alter_ego: AlterEgo
}
