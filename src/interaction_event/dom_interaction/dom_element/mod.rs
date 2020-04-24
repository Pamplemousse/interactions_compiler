use std::fmt::{Debug};

use serde::Deserialize;

pub mod tag;
use tag::Tag;


#[derive(Debug, Deserialize)]
pub struct DomElement {
    #[serde(alias="className")]
    pub class: String,
    #[serde(alias="id")]
    pub id: String,
    #[serde(alias="nodeName")]
    pub tag: Tag,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_an_element_properly() {
        let dom_element_as_json = r#"
            { "nodeName": "DIV"
            , "className": "class1 class2"
            , "id": "an-id"
            }
        "#;

        let dom_element: DomElement = serde_json::from_str(dom_element_as_json).unwrap();

        assert_eq!(dom_element.class, "class1 class2");
        assert_eq!(dom_element.id, "an-id");
        assert_eq!(dom_element.tag, Tag::DIV);
    }
}
