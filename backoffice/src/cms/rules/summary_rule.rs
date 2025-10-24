use cjtoolkit_structured_validator::types::description::description_alias::{
    Summary, SummaryError, SummaryRules,
};

#[inline]
fn summary_rule() -> SummaryRules {
    SummaryRules {
        is_mandatory: true,
        min_length: Some(5),
        max_length: Some(100),
    }
}

pub trait SummaryRuleExt {
    fn parse_summary(s: Option<&str>) -> Result<Summary, SummaryError>;
}

impl SummaryRuleExt for Summary {
    fn parse_summary(s: Option<&str>) -> Result<Summary, SummaryError> {
        Self::parse_custom(s, summary_rule())
    }
}
