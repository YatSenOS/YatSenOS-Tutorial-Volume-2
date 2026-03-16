use hashbrown::HashMap;
use spin::RwLock;

use super::*;
use crate::resource::ResourceSet;

#[derive(Debug, Clone)]
pub struct ProcessData {
    // shared data
    pub(super) env: Arc<RwLock<HashMap<String, String, ahash::RandomState>>>,
}

impl Default for ProcessData {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessData {
    pub fn new() -> Self {
        Self {
            env: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub fn env(&self, key: &str) -> Option<String> {
        self.env.read().get(key).cloned()
    }

    pub fn set_env(&mut self, key: &str, val: &str) {
        self.env.write().insert(key.into(), val.into());
    }
}
