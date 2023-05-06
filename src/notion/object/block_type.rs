use crate::notion::object::{
    Block,
    color::Color,
    rich_text::RichText,
    code_language::CodeLanguage
};

pub enum BlockType {
    Bookmark(Bookmark),
    Breadcrumb(Breadcrumb),
    BulletedListItem(BulletedListItem),
    Callout(Callout),
    ChildDatabase(ChildDatabase),
    ChildPage(ChildPage),
    Code(Code),
    Column(Column),
    ColumnList(ColumnList),
    Divider(Divider),
    Embed(Embed),
    Equation(Equation),
    File,
    Heading1(Heading),
    Heading2(Heading),
    Heading3(Heading),
    Image,
    LinkPreview,
    LinkToPage,
    NumberedListItem(NumberedListItem),
    Paragraph(Paragraph),
    Pdf,
    Quote(Quote),
    SyncedBlock,
    Table(Table),
    TableOfContents(TableOfContents),
    TableRow(TableRow),
    ToDo(ToDo),
    Toggle(Toggle),
    Video,
}

pub struct Bookmark {
    url: String,
    caption: vec![RichText]
}

impl Bookmark {
    pub fn new(url: String, caption: vec![RichText]) -> Self {
        Self {
            url,
            caption
        }
    }
}

pub struct Breadcrumb {}

pub struct BulletedListItem {
    rich_content: vec![RichText],
    color: Color,
    children: vec![Block]
}

pub struct Callout {
    rich_content: vec![RichText],
    color: Color,
}

pub struct ChildDatabase {
    title: String
}

pub struct ChildPage {
    title: String
}

pub struct Code {
    caption: vec![RichText],
    rich_content: vec![RichText],
    language: CodeLanguage
}

pub struct ColumnList {}
pub struct Column {}
pub struct Divider {}

pub struct Embed {
    url: string
}

pub struct Equation {
    expression: String
}

pub struct Heading {
    rich_content: vec![RichText],
    color: Color,
    isToggleable: bool
}

pub struct NumberedListItem {
    rich_content: vec![RichText],
    color: Color,
    children: vec![Block]
}

pub struct Paragraph {
    rich_content: vec![RichText],
    color: Color,
    children: vec![Block]
}

pub struct Quote {
    rich_content: vec![RichText],
    color: Color,
    children: vec![Block]
}

pub struct Table {
    table_width: i32,
    has_column_header: bool,
    has_row_header: bool,
}

pub struct TableRow {
    cells: vec![Block]
}

pub struct TableOfContents {
    color: Color,
}

pub struct ToDo {
    rich_content: vec![RichText],
    color: Color,
    children: vec![Block],
    checked: bool
}

pub struct Toggle {
    rich_content: vec![RichText],
    color: Color,
    children: vec![Block]
}
