use cjtoolkit_structured_validator::types::description::description_alias::{
    Text, TextError, TextRules,
};

fn markdown_rules() -> TextRules {
    TextRules {
        is_mandatory: true,
        min_length: None,
        max_length: None,
    }
}

pub trait MarkdownRulesExt {
    fn parse_markdown(s: Option<&str>) -> Result<Text, TextError>;
}

impl MarkdownRulesExt for Text {
    fn parse_markdown(s: Option<&str>) -> Result<Text, TextError> {
        Self::parse_custom(s, markdown_rules())
    }
}
