#[macro_use]
extern crate lazy_static;

use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::str::FromStr;

use clap::{ArgMatches, App, crate_authors, crate_name, crate_description, crate_version, load_yaml, Values};

mod code_generator;
mod html;
mod interaction_event;
use interaction_event::InteractionEvent;
use interaction_event::simplifier::simplify;
use interaction_event::simplifier::strategy::Strategy;
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

    let (input, mut output, pretty, mut strategies, url) = get_arguments(&matches)?;

    let mut interaction_events: Vec<InteractionEvent> =
        serde_json::from_reader(input).expect("JSON was not well-formatted");
    (*interaction_events).sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    simplify(&interaction_events, &mut strategies);

    let write: &dyn Fn(&Script) -> Result<String, serde_json::error::Error> =
        if pretty { &serde_json::to_string_pretty } else { &serde_json::to_string };
    let result = generate_zest_code_from(&interaction_events, url.to_string(), write)?;

    output.write_all(result.as_bytes())
}

type ArgumentsTuple<'a> = (Box<dyn BufRead>, Box<dyn Write>, bool, Vec<Strategy<'a>>, &'a str);
fn get_arguments<'a>(matches: &'a ArgMatches<'a>) -> Result<ArgumentsTuple<'a>, io::Error> {
    let input = get_input((*matches).value_of("input"))?;
    let output = get_output((*matches).value_of("output"))?;
    let pretty = (*matches).is_present("pretty");
    let strategies = get_strategies((*matches).values_of("strategies"))?;
    let url = (*matches).value_of("url")
        .unwrap_or_else(|| panic!("Expected `url` to be a required parameter."));

    Ok((
        input,
        output,
        pretty,
        strategies,
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

fn get_strategies(strategies_match: Option<Values>) -> Result<Vec<Strategy>, io::Error> {
    Ok(match strategies_match {
        Some(content) => {
            content
                .map(|y| { Strategy::from_str(y).unwrap_or_else(|e| panic!("{}", e)) })
                .collect::<Vec<Strategy>>()
        },
        None => vec![],
    })
}
