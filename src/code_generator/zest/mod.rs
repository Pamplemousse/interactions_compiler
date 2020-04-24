use serde::Serialize;

mod statement;
use statement::Statement;
use statement::action_print::ActionPrintStatement;


#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct Script {
    pub about: String,
    pub description: String,
    pub element_type: String,
    pub enabled: bool,
    pub index: usize,
    pub statements: Vec<Statement>,
    pub title: String,
    pub zest_version: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;

    #[test]
    fn serialize_a_zest_script_representation_properly() {
        let script :Script = Script {
            about: "This is a Zest script.".to_string(),
            description: "This is the description.".to_string(),
            element_type: "ZestScript".to_string(),
            enabled: true,
            index: 0,
            statements: vec![
                Statement::ActionPrint(ActionPrintStatement {
                    enabled: true,
                    message: "Hello!".to_string(),
                })
            ],
            title: "Title".to_string(),
            zest_version: "0.8".to_string(),
        };

        let generated_script = &(serde_json::to_string(&script).unwrap());

        let expected_result = r#"
            { "about": "This is a Zest script."
            , "description": "This is the description."
            , "elementType": "ZestScript"
            , "enabled": true
            , "index": 0
            , "statements":
              [ { "message": "Hello!"
                , "enabled": true
                , "elementType": "ZestActionPrint"
                }
              ]
            , "title": "Title"
            , "zestVersion": "0.8"
            }
        "#;

        assert_json_eq!(
            serde_json::from_str(generated_script).unwrap(),
            serde_json::from_str(expected_result).unwrap()
        );
    }
}
