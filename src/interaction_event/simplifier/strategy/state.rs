use std::mem::replace;

use crate::interaction_event::InteractionEvent;


pub enum State {
    SeveralEvents(Vec<InteractionEvent>),
    SingleEvent(Option<InteractionEvent>),
}

impl State {
    pub fn clear(self: &mut Self) {
        match self {
            State::SeveralEvents(interaction_events) => {
                // `std::vec::Vec` has a clear method that does what we want.
                interaction_events.clear()
            },
            State::SingleEvent(x) => {
                let _ = replace(x, None::<InteractionEvent>);
            },
        }
    }

    pub fn update(self: &mut Self, current_interaction_event: &InteractionEvent) {
        match self {
            State::SeveralEvents(interaction_events) => {
                interaction_events.push((*current_interaction_event).clone())
            },
            State::SingleEvent(x) => {
                let new_state = Some((*current_interaction_event).clone());
                let _ = replace(x, new_state);
            },
        }
    }
}
// /// Interact with the state used by the strategies.
// pub trait State {
//     fn clear(self: &mut Self) -> () where Self: Sized;
//     fn update(self: &mut Self, current_interaction_event: &InteractionEvent) -> () where Self: Sized;
// }

// impl State for Option<InteractionEvent> {
//     fn clear(self: &mut Self) -> () {
//         *self = None::<InteractionEvent>
//     }
//
//     fn update(self: &mut Self, current_interaction_event: &InteractionEvent) -> () {
//         *self = Some((*current_interaction_event).clone())
//     }
// }
//
// impl State for Vec<InteractionEvent> {
//     fn clear(self: &mut Self) -> () {
//         // `std::vec::Vec` has a clear method that does what we want.
//         self.clear()
//     }
//
//     fn update(self: &mut Self, current_interaction_event: &InteractionEvent) -> () {
//         self.push((*current_interaction_event).clone())
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    use crate::html::Tag;
    use crate::interaction_event::{Interaction, InteractionEvent};
    use crate::interaction_event::dom_interaction::DomInteraction;
    use crate::interaction_event::dom_interaction::dom_element::DomElement;
    use crate::interaction_event::dom_interaction::dom_event::DomEvent;

    #[test]
    fn a_vector_of_interaction_event_can_be_cleared() {
        let mut state: State = State::SeveralEvents(vec![InteractionEvent {
            interaction: Interaction::DomInteraction(DomInteraction {
                element: DomElement {
                    classes: vec!["class3".to_string()],
                    id: "another-id".to_string(),
                    tag: Tag::DIV,
                    value: None,
                },
                event: DomEvent::CLICK,
            }),
            timestamp: 12345,
        }]);

        state.clear();

        if let State::SeveralEvents(content) = state {
            assert!(content.is_empty());
        } else {
            panic!("The state should have been a `State::SeveralEvents`");
        }
    }

    #[test]
    fn update_on_a_vector_of_interaction_event_adds_a_new_element() {
        let mut state: State = State::SeveralEvents(vec![]);

        let interaction_event = InteractionEvent {
            interaction: Interaction::DomInteraction(DomInteraction {
                element: DomElement {
                    classes: vec!["class3".to_string()],
                    id: "another-id".to_string(),
                    tag: Tag::DIV,
                    value: None,
                },
                event: DomEvent::CLICK,
            }),
            timestamp: 12345,
        };

        state.update(&interaction_event);

        if let State::SeveralEvents(content) = state {
            assert_eq!(content, vec![interaction_event]);
        } else {
            panic!("The state should have been a `State::SeveralEvents`");
        }
    }

    #[test]
    fn an_optional_interaction_event_can_be_cleared() {
        let mut state: State = State::SingleEvent(Some(InteractionEvent {
            interaction: Interaction::DomInteraction(DomInteraction {
                element: DomElement {
                    classes: vec!["class3".to_string()],
                    id: "another-id".to_string(),
                    tag: Tag::DIV,
                    value: None,
                },
                event: DomEvent::CLICK,
            }),
            timestamp: 12345,
        }));

        state.clear();

        if let State::SingleEvent(content) = state {
            assert!(content.is_none());
        } else {
            panic!("The state should have been a `State::SingleEvent`");
        }
    }

    #[test]
    fn an_optional_interaction_event_containing_nothing_can_be_updated() {
        let mut state: State = State::SingleEvent(None);
        let interaction_event = InteractionEvent {
            interaction: Interaction::DomInteraction(DomInteraction {
                element: DomElement {
                    classes: vec!["class1".to_string(), "class2".to_string()],
                    id: "an-id".to_string(),
                    tag: Tag::INPUT,
                    value: Some("a-value".to_string()),
                },
                event: DomEvent::INPUT,
            }),
            timestamp: 12345,
        };

        state.update(&interaction_event);

        if let State::SingleEvent(content) = state {
            assert_eq!(content.unwrap(), interaction_event);
        } else {
            panic!("The state should have been a `State::SingleEvent`");
        }
    }

    #[test]
    fn an_optional_interaction_event_containing_a_previous_event_can_be_updated() {
        let mut state: State = State::SingleEvent(Some(InteractionEvent {
            interaction: Interaction::DomInteraction(DomInteraction {
                element: DomElement {
                    classes: vec!["class3".to_string()],
                    id: "another-id".to_string(),
                    tag: Tag::DIV,
                    value: None,
                },
                event: DomEvent::CLICK,
            }),
            timestamp: 12345,
        }));
        let interaction_event = InteractionEvent {
            interaction: Interaction::DomInteraction(DomInteraction {
                element: DomElement {
                    classes: vec!["class2".to_string(), "class3".to_string()],
                    id: "another-id".to_string(),
                    tag: Tag::INPUT,
                    value: Some("a-value".to_string()),
                },
                event: DomEvent::INPUT,
            }),
            timestamp: 12346,
        };

        state.update(&interaction_event);

        if let State::SingleEvent(content) = state {
            assert_eq!(content.unwrap(), interaction_event);
        } else {
            panic!("The state should have been a `State::SingleEvent`");
        }
    }
}
