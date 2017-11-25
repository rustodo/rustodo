use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Task {
    pub completed : bool,
    pub priority : Option<String>,
    pub completed_at : Option<DateTime<Utc>>,
    pub created_at : Option<DateTime<Utc>>,
    description : String,
    projects : Vec<String>,
    contexts : Vec<String>,
    options : HashMap<String,String>,
}

impl Task {
    pub fn new(description : &str) -> Task {
        Task {
            completed: false,
            priority: None,
            completed_at: None,
            created_at: None,
            description: String::from(description),
            projects: vec![],
            contexts: vec![],
            options: HashMap::new(),
        }
    }
}
