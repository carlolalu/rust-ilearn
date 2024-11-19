use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    username: String,
    content: String,
}

impl Message {
    pub fn new(username: &str, content: &str) -> Message {
        Message {
            username: username.to_string(),
            content: content.to_string(),
        }
    }

    pub fn uname(&self) -> String {
        self.username.clone()
    }

    pub fn craft_ack_msg(username: &str) -> Message {
        let content = format!("{username} just joined the chat");
        Message {
            username: "SERVER".to_string(),
            content,
        }
    }

    // TODO: error handling with Result
    pub fn from_serialized_buffer(buffer: &Vec<u8>) -> Message {
        let msg = serde_json::from_str(
            &String::from_utf8(buffer.clone())
                .expect("## The incoming msg could not be converted into a String: "),
        )
        .expect("## The incoming string could not be deserialized: ");
        msg
    }

    fn craft_final_msg(username: &str) -> Message {
        let content = format!("{username} just left the chat");
        Message {
            username: "SERVER".to_string(),
            content,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] :> {}", self.username, self.content)
    }
}

#[derive(Debug, Clone)]
pub struct Dispatch {
    userid: u64,
    msg: Message,
}

impl Dispatch {
    pub fn new(userid: u64, msg: Message) -> Dispatch {
        Dispatch { userid, msg }
    }

    pub fn extract_msg(self) -> Message {
        self.msg
    }

    pub fn uid(&self) -> u64 {
        self.userid
    }

    pub fn craft_final_dispatch(userid: u64, username: &str) -> Dispatch {
        Dispatch {
            userid,
            msg: Message::craft_final_msg(username),
        }
    }
}

impl fmt::Display for Dispatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[#{}] <{},{}>",
            self.userid, self.msg.username, self.msg.content
        )
    }
}
