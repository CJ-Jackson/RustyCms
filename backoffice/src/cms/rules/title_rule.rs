use cjtoolkit_structured_validator::types::name::name_alias::{Title, TitleError, TitleRules};

#[inline]
fn title_rule() -> TitleRules {
    TitleRules {
        is_mandatory: true,
        min_length: Some(5),
        max_length: Some(50),
    }
}

pub trait TitleRuleExt {
    fn parse_title(s: Option<&str>) -> Result<Title, TitleError>;
}

impl TitleRuleExt for Title {
    fn parse_title(s: Option<&str>) -> Result<Title, TitleError> {
        Self::parse_custom(s, title_rule())
    }
}
