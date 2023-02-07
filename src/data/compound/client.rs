#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Client {
    pub addr: String,
    pub username: String,
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: Hand,
    pub enabled_text_filtering: bool,
    pub allow_server_listings: bool,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            addr: Default::default(),
            username: Default::default(),
            locale: Default::default(),
            view_distance: Default::default(),
            chat_mode: ChatMode::Enabled,
            chat_colors: Default::default(),
            displayed_skin_parts: Default::default(),
            main_hand: Hand::Right,
            enabled_text_filtering: Default::default(),
            allow_server_listings: Default::default(),
        }
    }
}
