use serde::Serialize;

pub mod css_selector;
use css_selector::CssSelector;


#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all="camelCase")]
#[serde(tag="type")]
pub enum Selector {
    CssSelector(CssSelector),
    // TODO: Xpath
    // ...
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_include;

    use crate::html::Tag;

    #[test]
    fn serialize_a_css_selector_with_the_correct_type() {
        let selector :Selector = Selector::CssSelector(CssSelector {
            classes: vec!["class1".to_string(), "class2".to_string()],
            id: "an-id".to_string(),
            tag: Tag::DIV,
        });

        let generated_statement = &(serde_json::to_string(&selector).unwrap());
        let expected_result = r#"{ "type": "cssSelector" }"#;

        assert_json_include!(
            actual: serde_json::from_str(generated_statement).unwrap(),
            expected: serde_json::from_str(expected_result).unwrap()
        );
    }
}
