use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ClientWindowCloseStatement {
    pub enabled: bool,
    pub index: usize,
    pub sleep_in_seconds: u32,
    pub window_handle: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;

    #[test]
    fn serialize_a_client_window_close_statement_representation_properly() {
        let statement = ClientWindowCloseStatement {
            enabled: true,
            index: 100,
            sleep_in_seconds: 0,
            window_handle: "a-handle".to_string(),
        };

        let generated_statement = &(serde_json::to_string(&statement).unwrap());

        let expected_result = r#"
            { "enabled": true
            , "index": 100
            , "sleepInSeconds": 0
            , "windowHandle": "a-handle"
            }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_statement).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
