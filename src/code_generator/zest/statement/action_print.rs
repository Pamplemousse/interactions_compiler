use serde::Serialize;


#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ActionPrintStatement {
    pub enabled: bool,
    pub message: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;

    #[test]
    fn serialize_an_action_script_statement_representation_properly() {
        let statement = ActionPrintStatement {
            enabled: true,
            message: "Hello!".to_string(),
        };

        let generated_statement = &(serde_json::to_string(&statement).unwrap());

        let expected_result = r#"
            { "message": "Hello!"
            , "enabled": true
            }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_statement).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
