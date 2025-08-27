use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }
}
