use serde::Serialize;

use crate::code_generator::zest::statement::selector::Selector;


#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ClientElementSendKeysStatement {
    pub index: usize,
    pub value: String,
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
    fn serialize_a_client_element_send_keys_statement_representation_properly() {
        let statement = ClientElementSendKeysStatement {
            index: 1,
            selector: Selector::CssSelector(CssSelector {
                classes: vec!["class1".to_string(), "class2".to_string()],
                id: "an-id".to_string(),
                tag: Tag::DIV,
            }),
            value: "value-to-send".to_string(),
            window_handle: "a-handle".to_string(),
        };

        let generated_statement = &(serde_json::to_string(&statement).unwrap());

        let expected_result = r#"
		    { "element": "div#an-id.class1.class2"
			, "windowHandle": "a-handle"
            , "index": 1
			, "value": "value-to-send"
            , "type": "cssSelector"
            }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_statement).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
