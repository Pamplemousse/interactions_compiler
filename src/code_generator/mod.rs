pub mod zest;
use crate::interaction_event::InteractionEvent;
use zest::Script;


pub fn generate_zest_code_from(interaction_events :&[InteractionEvent], url: String) -> Result<String, serde_json::error::Error>
{
    let zest_script = Script::from_interaction_events(interaction_events, url);
    serde_json::to_string(&zest_script)
}
