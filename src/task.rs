use chrono::prelude::*;
use std::collections::HashMap;
use description_component::ComponentExtractor;
use description_component::DescriptionComponent;
use description_component::description_components_to_string;

#[derive(Debug, Clone)]
pub struct Task {
    pub completed : bool,
    pub priority : Option<char>,
    completed_at : Option<DateTime<Utc>>,
    created_at : Option<DateTime<Utc>>,
    description: Vec<DescriptionComponent>,
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
            description: description.extract_components(),
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

    pub fn description(&self) -> String {
        self.description.iter().map(|ref component| component.to_string()).collect()
    }

    pub fn projects(&self) -> Vec<String> {
        self.description.iter().filter_map(|component| match component {
            DescriptionComponent::Project(project) => Some(project.clone()),
            _ => None
        }).collect::<Vec<String>>()
    }

    pub fn contexts(&self) -> Vec<String> {
        self.description.iter().filter_map(|component| match component {
            DescriptionComponent::Context(context) => Some(context.clone()),
            _ => None
        }).collect::<Vec<String>>()
    }

    pub fn options(&self) -> HashMap<String,String> {
        self.description.iter().filter_map(|component| match component {
            DescriptionComponent::KeyValue(key, value) => Some((key.clone(), value.clone())),
            _ => None
        }).collect::<HashMap<String,String>>()
    }

    pub fn set_description(&mut self, description : &str) {
        self.description = description.extract_components()
    }

    pub fn description_components(&self) -> &Vec<DescriptionComponent> {
        &self.description
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        let completed = if self.completed {
            String::from("x ")
        } else {
            String::from("")
        };

        let priority = match self.priority {
            Some(p) => format!("({}) ", p),
            None => String::from("")
        };

        let completion = match self.completed_at {
            Some(date) => format!("{} ", date.format("%F")),
            None => String::from("")
        };

        let creation = match self.created_at {
            Some(date) => format!("{} ", date.format("%F")),
            None => String::from("")
        };

        format!("{completed}{priority}{completion}{creation}{description}",
                completed = completed,
                priority = priority,
                completion = completion,
                creation = creation,
                description = description_components_to_string(&self.description))
    }
}

#[cfg(test)]
mod tests {
    use task::Task;
    use description_component::DescriptionComponent;
    use chrono::prelude::*;

    #[test]
    fn can_create_task() {
        Task::new("I have something important to do!!");
    }

    #[test]
    fn chrono_can_parse_dates() {
        let datestring = "2017-11-25 00:00:00";
        let date = match Utc.datetime_from_str(&datestring, "%Y-%m-%d %H:%M:%S") {
            Ok(converted_date) => Some(converted_date.date()),
            Err(_) => None,
        };

        assert_ne!(date, None);
    }

    #[test]
    fn created_at_can_be_set_to_date_time() {
        let mut task = Task::new("Test");
        task.set_created_at(Some(Utc::now()));

        assert_ne!(task.created_at(), None);
    }

    #[test]
    fn completed_at_cannot_be_set_without_created_at() {
        let mut task = Task::new("Test");
        assert_eq!(task.set_completed_at(Some(Utc::now())), false);

        assert!(task.set_created_at(Some(Utc::now())));
        assert!(task.set_completed_at(Some(Utc::now())), false);
    }

    #[test]
    fn completed_at_can_be_set_to_datetime() {
        let mut task = Task::new("Test");
        assert!(task.set_created_at(Some(Utc::now())));
        assert!(task.set_completed_at(Some(Utc::now())), false);
    }

    #[test]
    fn print_task_with_just_description() {
        let task = Task::new("So many things to do.");
        assert_eq!(task.to_string(), "So many things to do.");
    }

    #[test]
    fn print_completed_task() {
        let mut task = Task::new("So many things to do.");
        task.completed = true;

        assert_eq!(task.to_string(), "x So many things to do.");
    }

    #[test]
    fn print_uncompleted_task_with_priority() {
        let mut task = Task::new("So many things to do.");
        task.priority = Some('A');
        assert_eq!(task.to_string(), "(A) So many things to do.");
    }

    #[test]
    fn print_uncompleted_task_with_creation_date() {
        let mut task = Task::new("So many things to do.");
        assert_eq!(task.set_created_at_from_str("2017-11-25"), true);
        assert_eq!(task.to_string(), "2017-11-25 So many things to do.");
    }

    #[test]
    fn print_completed_task_without_completion_date() {
        let mut task = Task::new("So many things to do.");
        task.set_created_at_from_str("2017-11-24");
        task.completed = true;

        assert_eq!(task.to_string(), "x 2017-11-24 So many things to do.");
    }

    #[test]
    fn print_completed_task_with_both_dates() {
        let mut task = Task::new("So many things to do.");
        task.set_created_at_from_str("2017-11-24");
        task.set_completed_at_from_str("2017-11-25");
        task.completed = true;

        assert_eq!(task.to_string(), "x 2017-11-25 2017-11-24 So many things to do.");
    }

    #[test]
    fn print_completed_task_with_both_dates_and_priority() {
        let mut task = Task::new("So many things to do.");
        task.set_created_at_from_str("2017-11-24");
        task.set_completed_at_from_str("2017-11-25");
        task.completed = true;
        task.priority = Some('B');

        assert_eq!(task.to_string(), "x (B) 2017-11-25 2017-11-24 So many things to do.");
    }

    #[test]
    fn can_set_dates_to_none_in_correct_order() {
        let mut task = Task::new("So many things to do.");
        task.set_created_at_from_str("2017-11-24");
        task.set_completed_at_from_str("2017-11-25");

        assert_ne!(task.created_at(), None);
        assert_ne!(task.completed_at(), None);

        assert!(task.set_completed_at(None));
        assert!(task.set_created_at(None));

        assert_eq!(task.created_at(), None);
        assert_eq!(task.completed_at(), None);
        assert_eq!(task.to_string(), "So many things to do.");
    }

    #[test]
    fn can_not_reset_creation_date_if_completion_date_is_set() {
        let mut task = Task::new("So many things to do.");

        task.set_created_at_from_str("2017-11-24");
        task.set_completed_at_from_str("2017-11-25");

        assert!(!(task.set_created_at(None)));
    }

    #[test]
    fn can_get_description_without_to_string() {
        let task = Task::new("So many things to do.");

        assert_eq!(task.description(), "So many things to do.");
    }

    #[test]
    fn description_is_not_altered_by_other_values() {
        let mut task = Task::new("So many things to do.");
        task.set_created_at_from_str("2017-11-24");
        task.set_completed_at_from_str("2017-11-25");
        task.completed = true;
        task.priority = Some('B');

        assert_eq!(task.description(), "So many things to do.");
    }

    #[test]
    fn description_can_be_set() {
        let mut task = Task::new("So many things to do.");
        task.set_description("So many things to do, but I'm too lazy!");

        assert_eq!(task.description(), "So many things to do, but I'm too lazy!");
    }

    #[test]
    fn setting_the_description_parses_tags() {
        let mut task = Task::new("");
        task.set_description("Description +project @context key:value more description.");

        assert_eq!("project", task.projects()[0]);
        assert_eq!("context", task.contexts()[0]);
        assert_eq!("value", task.options()["key"]);
    }

    #[test]
    fn get_components_and_their_position_in_the_description() {
        let task = Task::new("Description +project @context key:value more description.");
        let components = task.description_components();

        assert_eq!(components[0], DescriptionComponent::Text(String::from("Description ")));
        assert_eq!(components[1], DescriptionComponent::Project(String::from("project")));
        assert_eq!(components[2], DescriptionComponent::Text(String::from(" ")));
        assert_eq!(components[3], DescriptionComponent::Context(String::from("context")));
        assert_eq!(components[4], DescriptionComponent::Text(String::from(" ")));
        assert_eq!(components[5], DescriptionComponent::KeyValue(String::from("key"), String::from("value")));
        assert_eq!(components[6], DescriptionComponent::Text(String::from(" more description.")));
    }
}
