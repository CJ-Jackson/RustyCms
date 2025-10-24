use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
pub struct ComponentPositionForm {
    pub position: HashMap<u64, u64>,
}
