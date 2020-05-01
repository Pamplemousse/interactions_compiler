use std::io;

use clap::{App, crate_authors, crate_name, crate_description, crate_version, load_yaml};

mod code_generator;
mod html;
mod interaction_event;
use interaction_event::InteractionEvent;


fn main() {
    let yaml = load_yaml!("cli_arguments.yml");
    let matches = App::from_yaml(yaml)
        .about(crate_description!())
        .author(crate_authors!())
        .name(crate_name!())
        .version(crate_version!())
        .get_matches();

    let url = matches.value_of("url")
        // `url` is a required parameter.
        .unwrap()
        .to_string();

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
