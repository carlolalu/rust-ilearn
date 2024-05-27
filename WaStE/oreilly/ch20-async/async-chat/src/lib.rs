/// Following the mini project of chapter 20 regarding an asynchronous client/server architecture
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

/// The packet a client can send to the server.
/// The trait Serialize and Deserialize help us to
/// send such package across the network.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

/// The packet a server can send back to a client
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_fromclient_json() {
    use std::sync::Arc;

    let from_client = From_Client::Post {
        group_name: Arc::new("Dogs".to_string()),
        message: Arc::new("Samoyeds rock!".to_string()),
    };

    let json = serde_json::to_string(&from_client).unwrap();

    assert_eq!(
        json,
        r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
    );
    assert_eq!(
        serde_json::from_str::<FromClient>(&json).unwrap(),
        from_client
    );
}
