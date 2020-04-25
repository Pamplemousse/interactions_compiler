use serde::Serialize;


#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ClientLaunchStatement {
    pub browser_type: BrowserType,
    pub capabilities: String,
    pub enabled: bool,
    pub index: usize,
    pub url: String,
    pub window_handle: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all="lowercase")]
pub enum BrowserType {
    Firefox
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;

    #[test]
    fn serialize_a_client_launch_statement_representation_properly() {
        let statement = ClientLaunchStatement {
            browser_type: BrowserType::Firefox,
            capabilities: "key1=value1\nkey2=value2".to_string(),
            enabled: true,
            index: 1,
            url: "http://juice-shop.herokuapp.com".to_string(),
            window_handle: "a-handle".to_string(),
        };

        let generated_statement = &(serde_json::to_string(&statement).unwrap());

        let expected_result = r#"
            { "browserType": "firefox"
            , "capabilities": "key1=value1\nkey2=value2"
            , "enabled": true
            , "index": 1
            , "url": "http://juice-shop.herokuapp.com"
            , "windowHandle": "a-handle"
            }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_statement).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
