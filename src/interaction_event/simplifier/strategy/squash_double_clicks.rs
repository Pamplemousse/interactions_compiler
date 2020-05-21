use std::io;

use crate::interaction_event::Interaction;
use crate::interaction_event::InteractionEvent;
use super::state::State;
use super::{ACTION, PREDICATE};


/// * Replace the state by the current element **if** the state is empty and the current element is a `Click`.
/// * Remove the element corresponding to the state from the list, set the state to the current element **if**
/// the state is "similar" to the current element.
/// * Clear the state **if** the current element is not a `Click`.
lazy_static! {
    pub static ref CONDITIONAL_ACTIONS: Vec<(ACTION, PREDICATE)> = vec![
        (set_state_to_current as ACTION, state_is_none_and_current_is_click as PREDICATE),
        (remove_previous_from_list_and_set_state_to_current as ACTION, state_is_similar_to_current as PREDICATE),
        (clear_state as ACTION, current_is_not_a_click as PREDICATE),
    ];
}


fn state_should_be_single_event_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        "The `state` of the \"squash double clicks\" strategy should be a `State::SingleEvent`.",
    )
}

//
// Actions
//

fn set_state_to_current(state: &mut State, index: usize, new_interaction_events: &mut Vec<InteractionEvent>) -> Result<(), io::Error> {
    match state {
        State::SingleEvent(_) => {
            Ok(state.update(
                &new_interaction_events[index]
            ))
        },
        _ => Err(state_should_be_single_event_error()),
    }
}

fn remove_previous_from_list_and_set_state_to_current(state: &mut State, index: usize, new_interaction_events: &mut Vec<InteractionEvent>) -> Result<(), io::Error> {
    match state {
        State::SingleEvent(_) => {
            Ok(state.update(
                &new_interaction_events[index]
            ))
        },
        _ => Err(state_should_be_single_event_error()),
    }
}

fn clear_state(state: &mut State, _: usize, _: &mut Vec<InteractionEvent>) -> Result<(), io::Error> {
    match state {
        State::SingleEvent(_) => {
            Ok(state.clear())
        },
        _ => Err(state_should_be_single_event_error()),
    }
}

//
// Predicates
//

fn state_is_none_and_current_is_click(state: &State, current_interaction_event: &InteractionEvent) -> Result<bool, io::Error> {
    match state {
        State::SingleEvent(ref content) => {
            Ok(
                content.is_none() &&
                match &current_interaction_event.interaction {
                    Interaction::DomInteraction(interaction) => interaction.is_click(),
                }
            )
        },
        _ => Err(state_should_be_single_event_error()),
    }
}

fn state_is_similar_to_current(state: &State, current_interaction_event: &InteractionEvent) -> Result<bool, io::Error> {
    match state {
        State::SingleEvent(ref content) => {
            Ok(
                content.is_some() &&
                current_interaction_event.similar(content.as_ref().unwrap())
            )
        },
        _ => Err(state_should_be_single_event_error()),
    }
}

fn current_is_not_a_click(state: &State, current_interaction_event: &InteractionEvent) -> Result<bool, io::Error> {
    match state {
        State::SingleEvent(_) => {
            Ok(
                match &current_interaction_event.interaction {
                    Interaction::DomInteraction(interaction) => !interaction.is_click(),
                }
            )
        },
        _ => Err(state_should_be_single_event_error()),
    }
}
