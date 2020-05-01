use std::fmt::{Debug, Display, Formatter, Error};

use serde::Deserialize;


#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DomEvent {
    CLICK,
    KEYDOWN,
    KEYUP,
    INPUT,
}

impl Debug for DomEvent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            DomEvent::CLICK => write!(f, "click"),
            DomEvent::KEYDOWN => write!(f, "keydown"),
            DomEvent::KEYUP => write!(f, "keyup"),
            DomEvent::INPUT => write!(f, "input"),
        }
    }
}

impl Display for DomEvent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_click_properly() {
        let event_as_json = "\"click\"";
        let event: DomEvent = serde_json::from_str(event_as_json).unwrap();

        assert_eq!(event, DomEvent::CLICK);
    }

    #[test]
    fn deserializes_keydown_properly() {
        let event_as_json = "\"keydown\"";
        let event: DomEvent = serde_json::from_str(event_as_json).unwrap();

        assert_eq!(event, DomEvent::KEYDOWN);
    }

    #[test]
    fn deserializes_keyup_properly() {
        let event_as_json = "\"keyup\"";
        let event: DomEvent = serde_json::from_str(event_as_json).unwrap();

        assert_eq!(event, DomEvent::KEYUP);
    }

    #[test]
    fn deserializes_input_properly() {
        let event_as_json = "\"input\"";
        let event: DomEvent = serde_json::from_str(event_as_json).unwrap();

        assert_eq!(event, DomEvent::INPUT);
    }
}
