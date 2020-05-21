//! Each `Strategy` is the compound of an internal state, and a list of (predicate, action functions)
//! tuples, where "predicate" and "action" are both functions.

use std::io;
use std::str::FromStr;

mod squash_double_clicks;
mod state;
use crate::interaction_event::InteractionEvent;
use state::State;


/// The supported strategies.
pub enum Strategy<'a> {
    SquashDoubleClicks(&'a mut StrategyCore<'a>),
    // SquashDoubleKeydowns,
    // SquashKeysIntoInput,
}

/// An action uses the strategy state, and the current `InteractionEvent` to act on the list of
/// interaction events.
pub type ACTION = fn(&mut State, usize, &mut Vec<InteractionEvent>) -> Result<(), io::Error>;
/// A predicate uses the strategy state, and the current `InteractionEvent` to return a boolean.
pub type PREDICATE = fn(&State, &InteractionEvent) -> Result<bool, io::Error>;

pub struct StrategyCore {
    /// A set of actions to be performed under certain conditions.
    /// Applied in the order under which they are in the list.
    conditional_actions: &'static Vec<(ACTION, PREDICATE)>,
    /// Internal memory the strategy can use to keep track of stuff.
    state: &mut State,
}

impl Strategy {
    pub fn conditional_actions(&self) -> &Vec<(ACTION, PREDICATE)> {
        match &self {
            Strategy::SquashDoubleClicks(core) => core.conditional_actions,
        }
    }

    pub fn state(&mut self) -> &mut State {
        match &self {
            Strategy::SquashDoubleClicks(core) => &mut (core.state),
        }
    }
}

impl FromStr for Strategy {
    type Err = io::Error;

    fn from_str(strategy_name: &str) -> Result<Self, Self::Err> {
        match strategy_name {
            "squash_double_clicks" => Ok({
                let conditional_actions = &self::squash_double_clicks::CONDITIONAL_ACTIONS;
                let state = State::SingleEvent(None);

                Strategy::SquashDoubleClicks(
                    &mut StrategyCore { conditional_actions, state }
                )
            }),
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
