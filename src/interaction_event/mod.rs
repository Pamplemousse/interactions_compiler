use std::fmt::{Debug, Display, Formatter, Error};

use serde::Deserialize;

pub mod dom_interaction;
use dom_interaction::DomInteraction;


#[derive(Debug, Deserialize)]
pub struct InteractionEvent {
    #[serde(flatten)]
    pub interaction: Interaction,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
#[serde(tag="topic")]
pub enum Interaction {
    #[serde(rename="dom-events")]
    DomInteraction(DomInteraction)
}

impl Display for InteractionEvent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, f)
    }
}

impl Display for Interaction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::html::Tag;
    use dom_interaction::dom_event::DomEvent;

    #[test]
    fn deserializes_an_interaction_event_properly() {
        let interaction_event_as_json = r#"
            { "topic": "dom-events"
            , "element":
                { "nodeName": "DIV"
                , "className": "class1 class2"
                , "id": "an-id"
                , "value": "a-value"
                }
            , "event": "input"
            , "timestamp": 1586991500209
            }
        "#;

        let interaction_event: InteractionEvent = serde_json::from_str(interaction_event_as_json).unwrap();
        let timestamp = interaction_event.timestamp;

        match interaction_event.interaction {
            Interaction::DomInteraction(interaction) => {
                let dom_element = interaction.element;
                let event = interaction.event;

                assert_eq!(dom_element.classes, vec!["class1", "class2"]);
                assert_eq!(dom_element.id, "an-id");
                assert_eq!(dom_element.tag, Tag::DIV);
                assert_eq!(dom_element.value.unwrap(), "a-value".to_string());
                assert_eq!(event, DomEvent::INPUT);
                assert_eq!(timestamp, 1586991500209);
            }
        }
    }
}
