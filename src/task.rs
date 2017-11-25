use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Task {
    pub completed : bool,
    pub priority : Option<char>,
    completed_at : Option<DateTime<Utc>>,
    created_at : Option<DateTime<Utc>>,
    description : String,
    projects : Vec<String>,
    contexts : Vec<String>,
    options : HashMap<String,String>,
}

fn parse_datetime_str(datetime_str : &str) -> Option<DateTime<Utc>> {
    let datetime_str = format!("{} 00:00:00", datetime_str);

    match Utc.datetime_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
        Ok(datetime) => Some(datetime),
        Err(_) => None
    }
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

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }

    pub fn set_completed_at(&mut self, date_option : Option<DateTime<Utc>>) -> bool {
        match date_option {
            Some(_) => match self.created_at {
                //completed_at can only be set if there is a created_at
                Some(_) => {
                    self.completed_at = date_option;
                    true
                },
                None => false
            },
            None => {
                self.completed_at = None;
                true
            }
        }
    }

    pub fn set_completed_at_from_str(&mut self, datestring : &str) -> bool {
        match parse_datetime_str(datestring) {
            Some(datetime) => {
                self.set_completed_at(Some(datetime));
                true
            },
            None => false
        }
    }

    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }

    pub fn set_created_at(&mut self, date_option : Option<DateTime<Utc>>) -> bool {
        match date_option {
            Some(_) => {
                self.created_at = date_option;
                true
            },
            None => match self.completed_at {
                Some(_) => {
                    false
                },
                //created_at can only be set to None if completed_at is also None
                None => {
                    self.created_at = None;
                    true
                }
            }
        }
    }

    pub fn set_created_at_from_str(&mut self, datestring : &str) -> bool {
        match parse_datetime_str(datestring) {
            Some(datetime) => {
                self.set_created_at(Some(datetime));
                true
            },
            None => false
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description : &str) {
        self.description = description.to_owned();
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        let completed = if self.completed {
            "x ".to_owned()
        } else {
            "".to_owned()
        };

        let priority = match self.priority {
            Some(p) => format!("({}) ", p),
            None => "".to_owned()
        };

        let completion = match self.completed_at {
            Some(date) => format!("{} ", date.format("%F")),
            None => "".to_owned()
        };

        let creation = match self.created_at {
            Some(date) => format!("{} ", date.format("%F")),
            None => "".to_owned()
        };

        format!("{completed}{priority}{completion}{creation}{description}",
                completed = completed,
                priority = priority,
                completion = completion,
                creation = creation,
                description = self.description)
    }
}
