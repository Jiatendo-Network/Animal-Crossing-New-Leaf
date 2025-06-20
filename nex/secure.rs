use std::env;
use std::str::FromStr;

pub fn start_secure_server() {
    // Initialize the secure server
    globals::secure_server = Some(nex::new_prudp_server());
    let secure_server = globals::secure_server.as_mut().unwrap();
    secure_server.byte_stream_settings.use_structure_header = true;

    // Create and configure the secure endpoint
    globals::secure_endpoint = Some(nex::new_prudp_end_point(1));
    let secure_endpoint = globals::secure_endpoint.as_mut().unwrap();
    secure_endpoint.is_secure_end_point = true;
    secure_endpoint.server_account = globals::secure_server_account.clone();
    secure_endpoint.account_details_by_pid = globals::account_details_by_pid.clone();
    secure_endpoint.account_details_by_username = globals::account_details_by_username.clone();
    
    secure_server.bind_prudp_end_point(secure_endpoint.clone());

    // Configure server settings
    secure_server.library_versions.set_default(nex::LibraryVersion::new(3, 10, 1));
    secure_server.access_key = "d6f08b40".to_string();

    // Set up packet handler
    secure_endpoint.on_data(Box::new(|packet: &nex::PacketInterface| {
        let request = packet.rmc_message();

        println!("==animal crossing new leaf- Secure==");
        println!("Protocol ID: {}", request.protocol_id);
        println!("Method ID: {}", request.method_id);
        println!("===============");
    }));

    // Set up error handler
    secure_endpoint.on_error(Box::new(|err: &nex::Error| {
        globals::logger.error(&format!("Secure: {}", err));
    }));

    // Initialize matchmaking manager
    globals::matchmaking_manager = Some(common_globals::MatchmakingManager::new(
        secure_endpoint.clone(),
        globals::postgres.clone(),
    ));

    // Register protocols
    register_common_secure_server_protocols();

    // Get port from environment and start listening
    let port = env::var("PN_ACNL_SECURE_SERVER_PORT")
        .ok()
        .and_then(|p| u16::from_str(&p).ok())
        .unwrap_or_default();

    secure_server.listen(port);
}
