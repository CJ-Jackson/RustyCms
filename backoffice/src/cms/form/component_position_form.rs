use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct ComponentPositionForm {
    pub mapping: HashMap<u64, u64>,
}
