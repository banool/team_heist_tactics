use prost::Message;

pub fn get_message(key: String, message: impl Message) -> String {
    // Cheap janky self json.
    // String key and base64 encoding of the message.
    let mut buf = Vec::new();
    buf.reserve(message.encoded_len());
    message.encode(&mut buf).unwrap();
    let message_base64 = base64::encode(&buf);
    let message = format!("{{\"type\": \"{}\", \"body\": \"{}\"}}", key, message_base64);
    message
}
