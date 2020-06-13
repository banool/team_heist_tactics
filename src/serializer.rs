use prost::Message;
use std::any::type_name;
use log::debug;

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct WireMessage(pub String);

fn get_type_string<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn get_message(message: impl Message) -> WireMessage {
    // Cheap janky self json.
    // String key and base64 encoding of the message.
    let mut buf = Vec::new();
    buf.reserve(message.encoded_len());
    message.encode(&mut buf).unwrap();
    let message_type = get_type_string(&message);
    let message_base64 = base64::encode(&buf);
    let message = format!("{{\"message_type\": \"{}\", \"body\": \"{}\"}}", message_type, message_base64);
    debug!("Built client response message: {}", message);
    WireMessage(message)
}
