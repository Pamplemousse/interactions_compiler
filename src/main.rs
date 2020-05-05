use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

use clap::{ArgMatches, App, crate_authors, crate_name, crate_description, crate_version, load_yaml};

mod code_generator;
mod html;
mod interaction_event;
use interaction_event::InteractionEvent;
use code_generator::generate_zest_code_from;
use code_generator::zest::Script;


fn main() -> Result<(), io::Error> {
    let yaml = load_yaml!("cli_arguments.yml");
    let matches = App::from_yaml(yaml)
        .about(crate_description!())
        .author(crate_authors!())
        .name(crate_name!())
        .version(crate_version!())
        .get_matches();

    let (input, mut output, pretty, url) = get_arguments(&matches)?;

    let mut interaction_events: Vec<InteractionEvent> =
        serde_json::from_reader(input).expect("JSON was not well-formatted");
    (*interaction_events).sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let write: &dyn Fn(&Script) -> Result<String, serde_json::error::Error> =
        if pretty { &serde_json::to_string_pretty } else { &serde_json::to_string };
    let result = generate_zest_code_from(&interaction_events, url.to_string(), write)?;

    output.write_all(result.as_bytes())
}

type ArgumentsTuple<'a> = (Box<dyn BufRead>, Box<dyn Write>, bool, &'a str);
fn get_arguments<'a>(matches: &'a ArgMatches<'a>) -> Result<ArgumentsTuple<'a>, io::Error> {
    let input = get_input((*matches).value_of("input"))?;
    let output = get_output((*matches).value_of("output"))?;
    let pretty = (*matches).is_present("pretty");
    let url = (*matches).value_of("url")
        .unwrap_or_else(|| panic!("Expected `url` to be a required parameter."));

    Ok((
        input,
        output,
        pretty,
        url,
    ))
}

fn get_input(input_match: Option<&str>) -> Result<Box<dyn BufRead>, io::Error> {
    Ok(match input_match {
        Some(filename) => Box::new(BufReader::new(File::open(filename)?)),
        None => Box::new(BufReader::new(io::stdin())),
    })
}

fn get_output(output_match: Option<&str>) -> Result<Box<dyn Write>, io::Error> {
    Ok(match output_match {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(io::stdout()),
    })
}
