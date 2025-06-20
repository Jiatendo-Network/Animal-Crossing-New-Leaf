use std::collections::HashMap;

struct RMCMessage {
    protocol_id: u16,
    method_id: u16,
    call_id: u32,
    response_body: Vec<u8>,
}

impl RMCMessage {
    fn new(protocol_id: u16, method_id: u16, call_id: u32, response_body: Vec<u8>) -> Self {
        RMCMessage {
            protocol_id,
            method_id,
            call_id,
            response_body,
        }
    }
}

fn get_integer_settings(
    packet: &nex::PacketInterface,
    call_id: u32,
    integer_string_index: u32,
) -> (RMCMessage, Option<nex::Error>) {
    let mut map = HashMap::new();
    map.insert(0u16, 1i32);
    map.insert(1u16, 2i32);
    map.insert(2u16, 0i32);
    map.insert(3u16, 4i32);

    let mut rmc_response_stream = nex::ByteStreamOut::new(globals::secure_server::library_versions, globals::secure_server::byte_stream_settings);
    map.write_to(&mut rmc_response_stream);

    let rmc_response_body = rmc_response_stream.bytes().clone();

    let rmc_response = RMCMessage::new(
        utility::protocol_id,
        utility::method_get_integer_settings,
        call_id,
        rmc_response_body,
    );

    (rmc_response, None)
}


