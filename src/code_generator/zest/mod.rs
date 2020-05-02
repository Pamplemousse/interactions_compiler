use serde::Serialize;

mod statement;
use crate::interaction_event::InteractionEvent;
use statement::Statement;
use statement::client_launch::ClientLaunchStatement;
use statement::client_window_close::ClientWindowCloseStatement;


#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Script {
    pub about: String,
    pub description: String,
    pub element_type: String,
    pub enabled: bool,
    pub index: usize,
    pub statements: Vec<Statement>,
    pub title: String,
    pub zest_version: String,
}

impl Script {
    pub fn from_interaction_events(interaction_events: &[InteractionEvent], url: String) -> Self {
        let launch_window_statement = Statement::ClientLaunch(
            ClientLaunchStatement::from_url(url)
        );

        let window_handle = launch_window_statement.window_handle().unwrap();

        let statements_from_interactions: Vec<Statement> = (*interaction_events)
            .iter()
            .enumerate()
            .map(|(i, ie)| {
                // Index 0 is left for the top-level `ZestScript`;
                // Runs after `ClientLaunch`.
                let index = 1 + 1 + i;
                Statement::from_interaction_event(ie, index, window_handle.clone())
            })
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect();

        let window_close_statement = Statement::ClientWindowClose(
            ClientWindowCloseStatement::from_index_and_window_handle(
                // Index 0 is left for the top-level `ZestScript`;
                // Runs after all other statements: `ClientLaunch`, and all interactions.
                1 + 1 + (*interaction_events).len(),
                window_handle,
            )
        );

        let mut all_statements = vec![];
        all_statements.push(launch_window_statement);
        all_statements.extend(statements_from_interactions);
        all_statements.push(window_close_statement);

        Self {
            about: "This is a Zest script.".to_string(),
            description: "This is the description.".to_string(),
            element_type: "ZestScript".to_string(),
            enabled: true,
            index: 0,
            statements: all_statements,
            title: "Title".to_string(),
            zest_version: "0.8".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;
    use crate::html::Tag;
    use crate::interaction_event::{Interaction, InteractionEvent};
    use crate::interaction_event::dom_interaction::DomInteraction;
    use crate::interaction_event::dom_interaction::dom_element::DomElement;
    use crate::interaction_event::dom_interaction::dom_event::DomEvent;
    use statement::action_print::ActionPrintStatement;
    use statement::client_element_click::ClientElementClickStatement;
    use statement::client_element_send_keys::ClientElementSendKeysStatement;
    use statement::client_launch::BrowserType;
    use statement::selector::Selector;
    use statement::selector::css_selector::CssSelector;

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

    #[test]
    fn instantiate_a_zest_script_from_interaction_events_properly() {
        let interaction_events: Vec<InteractionEvent> = vec![
            InteractionEvent {
                interaction: Interaction::DomInteraction(DomInteraction {
                    element: DomElement {
                        classes: vec!["class1".to_string(), "class2".to_string()],
                        id: "an-id".to_string(),
                        tag: Tag::DIV,
                        value: None,
                    },
                    event: DomEvent::CLICK,
                }),
                timestamp: 12345,
            },
            InteractionEvent {
                interaction: Interaction::DomInteraction(DomInteraction {
                    element: DomElement {
                        classes: vec!["class2".to_string(), "class3".to_string()],
                        id: "another-id".to_string(),
                        tag: Tag::INPUT,
                        value: Some("a-value".to_string()),
                    },
                    event: DomEvent::INPUT,
                }),
                timestamp: 12346,
            },
        ];

        let url = "http://juice-shop.herokuapp.com".to_string();

        let generated_script = Script::from_interaction_events(&interaction_events, url.clone());

        // As the `window_handle` is generated, we need to retrieve it to construct the statements
        // to compare agains't.
        let generated_window_handle = ({
            match generated_script.statements.iter().nth(0).unwrap() {
                Statement::ClientLaunch(statement) => Some(statement.window_handle.clone()),
                _ => None,
            }
        } as Option<String>).unwrap();

        let expected_script :Script = Script {
            about: "This is a Zest script.".to_string(),
            description: "This is the description.".to_string(),
            element_type: "ZestScript".to_string(),
            enabled: true,
            index: 0,
            statements: vec![
                Statement::ClientLaunch(ClientLaunchStatement {
                    browser_type: BrowserType::Firefox,
                    capabilities: "".to_string(),
                    enabled: true,
                    index: 1,
                    url: url,
                    window_handle: generated_window_handle.clone(),
                }),
                Statement::ClientElementClick(ClientElementClickStatement {
                    index: 2,
                    selector: Selector::CssSelector(CssSelector {
                        classes: vec!["class1".to_string(), "class2".to_string()],
                        id: "an-id".to_string(),
                        tag: Tag::DIV,
                    }),
                    window_handle: generated_window_handle.clone(),
                }),
                Statement::ClientElementSendKeys(ClientElementSendKeysStatement {
                    index: 3,
                    selector: Selector::CssSelector(CssSelector {
                        classes: vec!["class2".to_string(), "class3".to_string()],
                        id: "another-id".to_string(),
                        tag: Tag::INPUT,
                    }),
                    value: "a-value".to_string(),
                    window_handle: generated_window_handle.clone(),
                }),
                Statement::ClientWindowClose(ClientWindowCloseStatement {
                    enabled: true,
                    index: 4,
                    sleep_in_seconds: 0,
                    window_handle: generated_window_handle.clone(),
                }),
            ],
            title: "Title".to_string(),
            zest_version: "0.8".to_string(),
        };

        assert_eq!(generated_script, expected_script);
    }
}
