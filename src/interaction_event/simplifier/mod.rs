//! Module aiming to provide necessary functions and tructures to help simplifying a `Vector` of
//! `InteractionEvent`.

use std::io;

pub mod strategy;
use crate::interaction_event::InteractionEvent;
use strategy::Strategy;


/// Remove `InteractionEvent`s from the input according to a given set of `Strategy`.
pub fn simplify(interaction_events: &[InteractionEvent], strategies: &[Strategy]) -> Result<Vec<InteractionEvent>, io::Error> {
    unimplemented!()
}
