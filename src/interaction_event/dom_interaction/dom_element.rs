use std::fmt::{Debug};

use serde::{Deserialize, Deserializer};

use crate::html::tag::Tag;


#[derive(Clone, Debug, Deserialize)]
pub struct DomElement {
    #[serde(alias="className", deserialize_with="deserialize_classes")]
    pub classes: Vec<String>,
    #[serde(alias="id")]
    pub id: String,
    #[serde(alias="nodeName")]
    pub tag: Tag,
    pub value: Option<String>,
}

fn deserialize_classes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(str_sequence
        .split(' ')
        .map(|item| item.to_owned())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_an_element_without_value_properly() {
        let dom_element_as_json = r#"
            { "nodeName": "DIV"
            , "className": "class1 class2"
            , "id": "an-id"
            }
        "#;

        let dom_element: DomElement = serde_json::from_str(dom_element_as_json).unwrap();

        assert_eq!(dom_element.classes, vec!["class1", "class2"]);
        assert_eq!(dom_element.id, "an-id");
        assert_eq!(dom_element.tag, Tag::DIV);
    }

    #[test]
    fn deserializes_an_element_with_value_properly() {
        let dom_element_as_json = r#"
            { "nodeName": "DIV"
            , "className": "class1 class2"
            , "id": "an-id"
            , "value": "a-value"
            }
        "#;

        let dom_element: DomElement = serde_json::from_str(dom_element_as_json).unwrap();

        assert_eq!(dom_element.classes, vec!["class1", "class2"]);
        assert_eq!(dom_element.id, "an-id");
        assert_eq!(dom_element.tag, Tag::DIV);
        assert_eq!(dom_element.value.unwrap(), "a-value".to_string());
    }
}
