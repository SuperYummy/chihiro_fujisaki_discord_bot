use on_message_creation::on_message_create;
use twilight_gateway::Event;
use crate::chihiro_fujisaki::ChihiroFujisaki;

mod on_message_creation;


pub async fn on_event(
    chihiro_fujisaki: &ChihiroFujisaki,
    event: Event
)
{

    match event {
        Event::Ready(_) => {

            println!("I am ready.");

        },
        Event::MessageCreate(message) => {

         let message_hope =  on_message_create(chihiro_fujisaki, *message).await;


         match message_hope {
            Ok(()) => {},
            Err(error) => {

                println!("{}", error);
                

            }
         }

        },
        _ => {}
    }

}
