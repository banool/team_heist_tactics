use anyhow::Result;
use prost::Message as ProstMessage;
use std::any::type_name;
use log::debug;

use crate::types::{MainMessage, GameState, InvalidRequest};
use crate::types::main_message::Body;
use actix::Message as ActixMessage;

#[derive(Clone, Debug)]
pub struct InternalMessage {
    pub main_message: MainMessage,
}

impl InternalMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(self.main_message.encoded_len());
        self.main_message.encode(&mut buf).unwrap();
        debug!("Built client response message: {:?}", buf);
        buf
    }

    pub fn from_bytes(wire_message: Vec<u8>) -> Result<InternalMessage> {
        let b: &[u8] = &wire_message;
        let main_message = match MainMessage::decode(b) {
            Ok(main_message) => main_message,
            Err(e) => return Err(anyhow::Error::new(e)),
        };
        Ok(InternalMessage{ main_message })
    }

    pub fn from_game_state(game_state: GameState) -> InternalMessage {
        let main_message =  MainMessage { body: Some(Body::GameState(game_state)) };
        InternalMessage { main_message }
    }

    pub fn from_invalid_reason(reason: String) -> InternalMessage {
        let invalid_request = InvalidRequest { reason };
        let main_message =  MainMessage { body: Some(Body::InvalidRequest(invalid_request)) };
        InternalMessage { main_message }
    }
}

impl ActixMessage for InternalMessage {
    type Result = ();
}