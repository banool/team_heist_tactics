use anyhow::Result;
use log::trace;
use prost::Message as ProstMessage;

use crate::types::main_message::Body;
use crate::types::{GameState, Internal, InvalidRequest, MainMessage};
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
        // trace!("Built client response message: {:?}", buf);
        buf
    }

    pub fn from_bytes(wire_message: &[u8]) -> Result<InternalMessage> {
        let main_message = match MainMessage::decode(wire_message) {
            Ok(main_message) => main_message,
            Err(e) => return Err(anyhow::Error::new(e)),
        };
        Ok(InternalMessage { main_message })
    }

    pub fn from_game_state(game_state: GameState) -> InternalMessage {
        let main_message = MainMessage {
            body: Some(Body::GameState(game_state.to_proto())),
        };
        InternalMessage { main_message }
    }

    pub fn from_invalid_reason(reason: String) -> InternalMessage {
        let invalid_request = InvalidRequest { reason };
        let main_message = MainMessage {
            body: Some(Body::InvalidRequest(invalid_request.to_proto())),
        };
        InternalMessage { main_message }
    }
}

impl ActixMessage for InternalMessage {
    type Result = ();
}
