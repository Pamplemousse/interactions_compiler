use std::fmt::{Debug, Display, Formatter, Error};

use serde;
use serde::Deserialize;


#[derive(Deserialize, PartialEq)]
pub enum Tag {
    BUTTON,
    DIV,
    INPUT,
    SPAN,
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Tag::BUTTON => write!(f, "button"),
            Tag::DIV => write!(f, "div"),
            Tag::INPUT => write!(f, "input"),
            Tag::SPAN => write!(f, "span"),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_button_properly() {
        let tag_as_json = "\"BUTTON\"";
        let tag: Tag = serde_json::from_str(tag_as_json).unwrap();

        assert_eq!(tag, Tag::BUTTON);
    }

    #[test]
    fn deserializes_div_properly() {
        let tag_as_json = "\"DIV\"";
        let tag: Tag = serde_json::from_str(tag_as_json).unwrap();

        assert_eq!(tag, Tag::DIV);
    }

    #[test]
    fn deserializes_input_properly() {
        let tag_as_json = "\"INPUT\"";
        let tag: Tag = serde_json::from_str(tag_as_json).unwrap();

        assert_eq!(tag, Tag::INPUT);
    }

    #[test]
    fn deserializes_span_properly() {
        let tag_as_json = "\"SPAN\" ";
        let tag: Tag = serde_json::from_str(tag_as_json).unwrap();

        assert_eq!(tag, Tag::SPAN);
    }
}
