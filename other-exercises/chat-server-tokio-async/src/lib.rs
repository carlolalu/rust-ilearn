use std::fmt;

use serde::{Deserialize, Serialize};

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, GenericError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    username: String,
    content: String,
}

pub enum UserStatus {
    Present,
    Absent,
}

impl Message {
    pub fn new(username: &str, content: &str) -> Message {
        Message {
            username: username.to_string(),
            content: content.to_string(),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn from_serialized_buffer(buffer: &Vec<u8>) -> Result<Message> {
        let msg = serde_json::from_str::<Message>(&String::from_utf8(buffer.clone())?)?;
        Ok(msg)
    }

    pub fn craft_status_change_msg(username: &str, status: UserStatus) -> Message {
        let content = match status {
            UserStatus::Present => format!("{username} just joined the chat"),
            UserStatus::Absent => format!("{username} just left the chat"),
        };

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

    pub fn into_msg(self) -> Message {
        self.msg
    }

    pub fn get_userid(&self) -> u64 {
        self.userid
    }

    pub fn craft_status_change_dispatch(
        userid: u64,
        username: &str,
        user_status: UserStatus,
    ) -> Dispatch {
        Dispatch {
            userid,
            msg: Message::craft_status_change_msg(username, user_status),
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
