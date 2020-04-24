use serde::Serialize;

pub mod action_print;
use action_print::ActionPrintStatement;


#[derive(Clone, Serialize)]
#[serde(tag="elementType")]
pub enum Statement {
    #[serde(rename="ZestActionPrint")]
    ActionPrint(ActionPrintStatement),
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_include;

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
}
