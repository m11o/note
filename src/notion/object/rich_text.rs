use crate::notion::object::color::Color;

pub enum RichTextType {
    Text(RichTextText),
    Mention(RichTextMention),
    Equation(RichTextEquation),
}

struct RichTextText {
    content: String,
    link: Option<String>,
}

// 不要の可能性があるので、一旦未実装
struct RichTextMention {}

struct RichTextEquation {
    expression: String,
}

struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
    code: bool,
    color: Color,
}

pub struct RichText {
    content: RichTextType,
    plain_text: String,
    href: Option<String>,
    annotations: Annotations,
}
