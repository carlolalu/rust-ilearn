use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Dispatch {
    UserMessage {id: String, text: String},
    IdRequestResponse {requested_id : String, accepted : bool},  //
}