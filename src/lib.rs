use std::io::{BufRead};
use wasm_bindgen::prelude::*;

mod code_generator;
mod html;
mod interaction_event;
use interaction_event::InteractionEvent;
use code_generator::{ResultStringOrError, generate_zest_code_from};
use code_generator::zest::Script;


pub enum Input<'a> {
  Str(&'a str),
  BufRead(Box<dyn BufRead>),
}

#[wasm_bindgen]
pub fn zest_from(json: &str, url: &str, pretty: bool) -> String {
    match compile(Input::Str(json), url, pretty) {
        Ok(zest_script) => zest_script,
        Err(error) => error.to_string(),
    }
}

pub fn compile(json: Input, url: &str, pretty: bool) -> ResultStringOrError {
    let mut interaction_events: Vec<InteractionEvent> =
        (match json {
            Input::Str(input) => serde_json::from_str(input),
            Input::BufRead(input) => serde_json::from_reader(input),
        }).expect("JSON was not well-formatted");

    (*interaction_events).sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let write: &dyn Fn(&Script) -> Result<String, serde_json::error::Error> =
        if pretty { &serde_json::to_string_pretty } else { &serde_json::to_string };

    generate_zest_code_from(&interaction_events, url.to_string(), write)
}
