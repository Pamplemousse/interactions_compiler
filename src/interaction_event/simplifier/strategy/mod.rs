use std::io;
use std::str::FromStr;


/// The supported strategies.
pub enum Strategy {
    SquashDoubleClicks,
}

impl FromStr for Strategy {
    type Err = io::Error;

    fn from_str(strategy_name: &str) -> Result<Self, Self::Err> {
        match strategy_name {
            "squash_double_clicks" => Ok(Strategy::SquashDoubleClicks),
            &_ => Err(io::Error::new(
               io::ErrorKind::Other,
               "Expected only values corresponding to the `Strategy` enum to be passed by the CLI."
            )),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_strategies_from_strings() {
        let strategies: Vec<Strategy> = [
            "squash_double_clicks",
        ].iter()
            .map(|x| Strategy::from_str(x).unwrap())
            .collect();
    }
}
