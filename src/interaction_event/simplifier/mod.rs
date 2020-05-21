//! Module aiming to provide necessary functions and tructures to help simplifying a `Vector` of
//! `InteractionEvent`.

use std::io;

pub mod strategy;
use crate::interaction_event::InteractionEvent;
use strategy::Strategy;


/// Remove `InteractionEvent`s from the input according to a given set of `Strategy`.
pub fn simplify(interaction_events: &Vec<InteractionEvent>, strategies: &mut [Strategy]) -> Result<Vec<InteractionEvent>, io::Error> {
    let mut new_interaction_events: Vec<InteractionEvent> = Vec::new();

    for (index, current_interaction_event) in interaction_events.iter().enumerate() {
        for strategy in &mut *strategies {

            let conditional_actions = strategy.conditional_actions();

            // for (action, condition) in conditional_actions {
            //
            //     if condition(state, current_interaction_event)? {
            //         action(state, index, &mut new_interaction_events)?
            //     }
            //     // println!("{} -> {}", index, current_interaction_event);
            // }
        }
    }

    Ok(new_interaction_events)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_deduplicate_multiple_clicks_with_close_timestamps() {
        // Setup the list of interaction events from JSON for readability.
        let interaction_event_as_json = r#"[
          { "topic": "dom-events"
          , "element": { "nodeName": "DIV", "classes": "", "id": "an-id", "value": "a-value" }
          , "event": "click"
          , "timestamp": 1586991500209
          }
        , { "topic": "dom-events"
          , "element": { "nodeName": "DIV", "classes": "", "id": "an-id", "value": "a-value" }
          , "event": "click"
          , "timestamp": 1586991500210
          }
        , { "topic": "dom-events"
          , "element": { "nodeName": "DIV", "classes": "", "id": "an-id", "value": "a-value" }
          , "event": "input"
          , "timestamp": 1586991500210
          }
        ]"#;
        let interaction_events: Vec<InteractionEvent> =
            serde_json::from_str(interaction_event_as_json).unwrap();
        let mut strategies = [Strategy::from_str("squash_double_clicks").unwrap()];

        let expected_result: Vec<InteractionEvent> = interaction_events[1..interaction_events.len()].to_vec();

        simplify(&interaction_events, &mut strategies);

        assert_eq!(1, 1);
    }
}
