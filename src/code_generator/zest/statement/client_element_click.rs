use serde::Serialize;

use crate::code_generator::zest::statement::selector::Selector;


#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ClientElementClickStatement {
    pub index: usize,
    #[serde(flatten)]
    pub selector: Selector,
    pub window_handle: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;

    use crate::code_generator::zest::statement::selector::css_selector::CssSelector;
    use crate::html::Tag;

    #[test]
    fn serialize_a_client_element_click_statement_representation_properly() {
        let statement = ClientElementClickStatement {
            index: 1,
            selector: Selector::CssSelector(CssSelector {
                classes: vec!["class1".to_string(), "class2".to_string()],
                id: "an-id".to_string(),
                tag: Tag::DIV,
            }),
            window_handle: "a-handle".to_string(),
        };

        let generated_statement = &(serde_json::to_string(&statement).unwrap());

        let expected_result = r#"
		    { "element": "div#an-id.class1.class2"
            , "index": 1
            , "windowHandle": "a-handle"
            , "type": "cssSelector"
            }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_statement).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
