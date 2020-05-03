pub mod zest;
use crate::interaction_event::InteractionEvent;
use zest::Script;


type ResultStringOrError = Result<String, serde_json::error::Error>;
pub fn generate_zest_code_from(
    interaction_events :&[InteractionEvent],
    url: String,
    write: &dyn Fn(&Script) -> ResultStringOrError
) -> ResultStringOrError {
    write(
        &Script::from_interaction_events(interaction_events, url)
    )
}
