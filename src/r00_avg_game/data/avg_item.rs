pub struct AvgItem {
    pub background_music: Option<String>,
    pub background_image_path: Option<String>,
    pub center_character_image_path: Option<String>,
    pub character_name: Option<String>,
    pub content: Option<String>,
    pub selection_items: Option<Vec<SelectionItem>>,
    pub next_index: Option<u32>,
}

pub struct SelectionItem {
    pub content: String,
    pub next_index: u32,
}