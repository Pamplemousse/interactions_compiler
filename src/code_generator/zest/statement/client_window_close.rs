use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ClientWindowCloseStatement {
    pub enabled: bool,
    pub index: usize,
    pub sleep_in_seconds: u32,
    pub window_handle: String,
}

impl ClientWindowCloseStatement {
    pub fn from_index_and_window_handle(index: usize, window_handle: String) -> Self {
        Self {
            enabled: true,
            index,
            sleep_in_seconds: 0,
            window_handle,
        }
    }
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

    #[test]
    fn instantiate_from_an_index_and_a_window_handle() {
        let index = 100;
        let window_handle = "a-handle".to_string();

        let statement = ClientWindowCloseStatement::from_index_and_window_handle(
            index,
            window_handle,
        );

        let expected_statement = ClientWindowCloseStatement {
            enabled: true,
            index: 100,
            sleep_in_seconds: 0,
            window_handle: "a-handle".to_string(),
        };

        assert_eq!(statement, expected_statement);
    }
}
