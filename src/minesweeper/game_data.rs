#[derive(Clone)]
pub struct GameData {
    pub image_data: Vec<u8>,
    pub total_actions: u8,
    pub opened_fields: u8,
    pub closed_fields: u8,
    pub total_fields: u8,
    pub mine_count: u8,
}
