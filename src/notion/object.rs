mod block_type;
mod rich_text;
mod color;
mod code_language;

use block_type::BlockType;

pub struct Page {
    id: String,
    created_time: String,
    last_edited_time: Option<String>,
    archived: bool,
    url: String,
    title: String
}

impl Page {
    pub fn new(id: String, title: String, created_time: String, archived: bool, url: String, last_edited_time: Option<String>) -> Self {
        Self {
            id,
            title,
            created_time,
            last_edited_time,
            archived,
            url
        }
    }
}

pub struct Block {
    id: String,
    created_time: String,
    last_edited_time: Option<String>,
    archived: bool,
    has_children: bool,
    object: BlockType
}

impl Block {
    pub fn new(id: String, created_time: String, archived: bool, has_children: bool, object: BlockType, last_edited_time: Option<String>) -> Self {
        Self {
            id,
            created_time,
            last_edited_time,
            archived,
            has_children,
            object
        }
    }
}
