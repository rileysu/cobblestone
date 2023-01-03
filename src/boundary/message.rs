use tokio::sync::mpsc;

#[derive(Debug)]
pub enum InboundMessage {
    InitConnection {
        outbound_tx: mpsc::UnboundedSender<OutboundMessage>,
    },
    ServerInformationRequest,
    PingRequest {
        payload: i64,
    },
    LoginStart {
        username: String,
        uuid: u128,
    },
    TermConnection,
}

#[derive(Debug)]
pub struct IdentifiedInboundMessage {
    pub id: String,
    pub message: InboundMessage,
}

#[derive(Debug)]
pub enum OutboundMessage {
    ServerInformationResponse {
        version_name: String,
        version_protocol: u32,
        players_max: usize,
        players_online: usize,
        sample: Vec<(String, String)>,
        description_text: String, //Chat object but can be added later
        favicon: String, //png as base64 but can be added later
        previews_chat: bool,
        enforce_secure_chat: bool,
    },
    PingResponse {
        payload: i64,
    },
    LoginSuccess {
        username: String,
        uuid: u128,
        properties: Vec<LoginProperty>,
    },
    Login {
        entity_id: i32,
        is_hardcore: bool,
        gamemode: u8,
        previous_gamemode: i8,
        dimension_names: Vec<(String, String)>,
    }
}

#[derive(Debug)]
pub struct LoginProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}