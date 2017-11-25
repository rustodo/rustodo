extern crate chrono;

mod task;

#[cfg(test)]
mod tests {
    use task::Task;
    use chrono::prelude::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_create_task() {
        Task::new("I have something important to do!!");
    }

    #[test]
    fn chrono_can_parse_dates() {
        let datestring = "2017-11-25 00:00:00";
        let date = match Utc.datetime_from_str(&datestring, "%Y-%m-%d %H:%M:%S") {
            Ok(converted_date) => Some(converted_date.date()),
            Err(error) => {
                println!("THE DAMN ERROR WAS: {}", error);
                None
            }
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
}

