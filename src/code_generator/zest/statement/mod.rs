use serde::Serialize;

pub mod action_print;
pub mod client_element_click;
pub mod client_element_send_keys;
pub mod client_launch;
pub mod selector;
use action_print::ActionPrintStatement;
use client_element_click::ClientElementClickStatement;
use client_element_send_keys::ClientElementSendKeysStatement;
use client_launch::ClientLaunchStatement;


#[derive(Clone, Serialize)]
#[serde(tag="elementType")]
pub enum Statement {
    #[serde(rename="ZestActionPrint")]
    ActionPrint(ActionPrintStatement),
    #[serde(rename="ZestClientLaunch")]
    ClientLaunch(ClientLaunchStatement),
    #[serde(rename="ZestClientElementClick")]
    ClientElementClick(ClientElementClickStatement),
    #[serde(rename="ZestClientElementSendKeys")]
    ClientElementSendKeys(ClientElementSendKeysStatement),
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_include;

    use crate::html::Tag;
    use client_launch::BrowserType;
    use selector::Selector;
    use selector::css_selector::CssSelector;

    #[test]
    fn serialize_an_action_print_statement_representation_with_the_right_element_type() {
        let statement :Statement = Statement::ActionPrint(ActionPrintStatement {
            enabled: true,
            message: "Hello!".to_string(),
        });

        let generated_statement = &(serde_json::to_string(&statement).unwrap());
        let expected_result = r#"{ "elementType": "ZestActionPrint" }"#;

        assert_json_include!(
            actual: serde_json::from_str(generated_statement).unwrap(),
            expected: serde_json::from_str(expected_result).unwrap()
        );
    }

    #[test]
    fn serialize_a_client_element_click_statement_representation_with_the_right_element_type() {
        let statement :Statement = Statement::ClientElementClick(ClientElementClickStatement {
            index: 1,
            selector: Selector::CssSelector(CssSelector {
                classes: vec!["class1".to_string(), "class2".to_string()],
                id: "an-id".to_string(),
                tag: Tag::DIV,
            }),
            window_handle: "a-handle".to_string(),
        });

        let generated_statement = &(serde_json::to_string(&statement).unwrap());
        let expected_result = r#"{ "elementType": "ZestClientElementClick" }"#;

        assert_json_include!(
            actual: serde_json::from_str(generated_statement).unwrap(),
            expected: serde_json::from_str(expected_result).unwrap()
        );
    }

    #[test]
    fn serialize_a_client_element_send_keys_statement_representation_with_the_right_element_type() {
        let statement :Statement = Statement::ClientElementSendKeys(ClientElementSendKeysStatement {
            index: 1,
            selector: Selector::CssSelector(CssSelector {
                classes: vec!["class1".to_string(), "class2".to_string()],
                id: "an-id".to_string(),
                tag: Tag::INPUT,
            }),
            value: "value-to-send".to_string(),
            window_handle: "a-handle".to_string(),
        });

        let generated_statement = &(serde_json::to_string(&statement).unwrap());
        let expected_result = r#"{ "elementType": "ZestClientElementSendKeys" }"#;

        assert_json_include!(
            actual: serde_json::from_str(generated_statement).unwrap(),
            expected: serde_json::from_str(expected_result).unwrap()
        );
    }

    #[test]
    fn serialize_a_launch_client_statement_representation_with_the_right_element_type() {
        let statement :Statement = Statement::ClientLaunch(ClientLaunchStatement {
            browser_type: BrowserType::Firefox,
            capabilities: "key1=value1\nkey2=value2".to_string(),
            enabled: true,
            index: 1,
            url: "http://juice-shop.herokuapp.com".to_string(),
            window_handle: "a-handle".to_string(),
        });

        let generated_statement = &(serde_json::to_string(&statement).unwrap());
        let expected_result = r#"{ "elementType": "ZestClientLaunch" }"#;

        assert_json_include!(
            actual: serde_json::from_str(generated_statement).unwrap(),
            expected: serde_json::from_str(expected_result).unwrap()
        );
    }
}
