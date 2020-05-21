use std::fmt::{Debug};

use serde::Deserialize;

pub mod dom_element;
pub mod dom_event;
pub use dom_element::DomElement;
use dom_event::DomEvent;


#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DomInteraction {
    pub element: DomElement,
    pub event: DomEvent,
}

impl DomInteraction {
    pub fn is_click(&self) -> bool {
        match self.event {
            DomEvent::CLICK => true,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_a_dom_interaction_properly() {
        let dom_interaction_as_json = r#"
            { "element":
              { "nodeName": "DIV"
              , "className": "class1 class2"
              , "id": "an-id"
              }
            , "event": "click"
            }
        "#;

        let dom_interaction: DomInteraction = serde_json::from_str(dom_interaction_as_json).unwrap();

        assert_eq!(dom_interaction.element.classes, vec!["class1", "class2"]);
        assert_eq!(dom_interaction.element.id, "an-id");
        assert_eq!(dom_interaction.element.tag, crate::html::Tag::DIV);
        assert_eq!(dom_interaction.event, dom_event::DomEvent::CLICK);
    }
}
