use std::env;
use std::num::ParseIntError;

fn register_common_authentication_server_protocols() -> Result<(), ParseIntError> {
    let ticket_granting_protocol = ticketgranting::new_protocol();
    globals::authentication_endpoint.register_service_protocol(ticket_granting_protocol);
    let common_ticket_granting_protocol = commonticketgranting::new_common_protocol(ticket_granting_protocol);
    common_ticket_granting_protocol.set_pretendo_validation(globals::aes_key);

    let port: u16 = env::var("JN_ACNL_SECURE_SERVER_PORT")?.parse()?;

    let mut secure_station_url = types::StationURL::new("");
    secure_station_url.set_url_type(constants::STATION_URL_PRUDPS);
    secure_station_url.set_address(env::var("JN_ACNL_SECURE_SERVER_HOST")?);
    secure_station_url.set_port_number(port);
    secure_station_url.set_connection_id(1);
    secure_station_url.set_principal_id(types::JID::new(2));
    secure_station_url.set_stream_id(1);
    secure_station_url.set_stream_type(constants::STREAM_TYPE_RV_SECURE);
    secure_station_url.set_type(constants::STATION_URL_FLAG_PUBLIC as u8);

    common_ticket_granting_protocol.secure_station_url = secure_station_url;
    common_ticket_granting_protocol.build_name = types::String::new("branch:origin/release/ngs/3.10.x.200x build:3_10_22_2006_0");
    common_ticket_granting_protocol.secure_server_account = globals::secure_server_account;

    Ok(())
}

