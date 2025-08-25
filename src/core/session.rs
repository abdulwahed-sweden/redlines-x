use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub target: String,
    pub cookies: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub variables: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(target: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            target: target.to_string(),
            cookies: HashMap::new(),
            headers: HashMap::new(),
            variables: HashMap::new(),
            created_at: Utc::now(),
        }
    }
    
    pub fn add_cookie(&mut self, name: &str, value: &str) {
        self.cookies.insert(name.to_string(), value.to_string());
    }
    
    pub fn get_cookie(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }
}
