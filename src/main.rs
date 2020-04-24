use std::io;

mod interaction_event;
use interaction_event::InteractionEvent;


fn main() {
    let stdin = io::stdin();

    let interaction_events: Vec<InteractionEvent> =
        serde_json::from_reader(stdin).expect("JSON was not well-formatted");

    for event in interaction_events {
        println!(
            "interaction: {}, at {}",
            event.interaction,
            event.timestamp
        );
    }
}
