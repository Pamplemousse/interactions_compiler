use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::html::Tag;


#[derive(Clone)]
pub struct CssSelector {
    pub classes: Vec<String>,
    pub id: String,
    pub tag: Tag,
}

impl Serialize for CssSelector {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized_selector = format!("{}#{}.{}", &self.tag, &self.id, &self.classes.join("."));

        let mut serializer = s.serialize_struct("element", 1)?;
        serializer.serialize_field("element", &serialized_selector)?;
        serializer.end()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;

    #[test]
    fn serialize_a_css_selector_properly() {
        let css_selector = CssSelector {
            classes: vec!["class1".to_string(), "class2".to_string()],
            id: "an-id".to_string(),
            tag: Tag::DIV,
        };

        let generated_css_selector = &(serde_json::to_string(&css_selector).unwrap());

        let expected_result = r#"
            { "element": "div#an-id.class1.class2" }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_css_selector).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
