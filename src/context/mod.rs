use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    pub user_id: String,
    pub settings: serde_json::Value,
    pub history: VecDeque<String>,
}

pub struct ContextManager {
    capacity: usize,
    // In a real scenario, this would likely be persistent storage
    pub contexts: HashMap<String, UserContext>,
}

impl ContextManager {
    pub fn new(capacity: usize) -> Self {
        ContextManager {
            capacity,
            contexts: HashMap::new(),
        }
    }

    pub fn get_or_create_context(&mut self, user_id: &str) -> &mut UserContext {
        self.contexts.entry(user_id.to_string()).or_insert_with(|| UserContext {
            user_id: user_id.to_string(),
            settings: serde_json::json!({}),
            history: VecDeque::with_capacity(self.capacity),
        })
    }

    pub fn add_to_history(&mut self, user_id: &str, command: String) {
        let capacity = self.capacity;
        let context = self.get_or_create_context(user_id);
        if context.history.len() == capacity {
            context.history.pop_front();
        }
        context.history.push_back(command);
    }
}


