use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Dispatch {
    UserMessage {
        id: String,
        text: String,
    },
    IdRequestResponse {
        requested_id: String,
        accepted: bool,
    },
}

impl std::fmt::Display for Dispatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dispatch::UserMessage { id, text } => {
                write!(f, r##"{}:> {}"##, *id, *text)
            }
            Dispatch::IdRequestResponse {
                requested_id,
                accepted,
            } => {
                let status = match *accepted {
                    true => "accepted",
                    false => "refused",
                };
                write!(
                    f,
                    r##"server:> the username "{}" is {}"##,
                    *requested_id, status
                )
            }
        }
    }
}
