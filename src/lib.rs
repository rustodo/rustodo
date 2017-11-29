extern crate chrono;
extern crate regex;
#[macro_use] extern crate lazy_static;

mod task;
mod tokens;
mod tags;

#[cfg(test)]
mod tests {
    use task::Task;
    use chrono::prelude::*;
    use tokens::Tokenizer;
    use tags::TagExtractor;
    use tags::Tag;

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

    #[test]
    #[ignore]
    fn learning_test_tokens_work() {
        print!("\n {:?} \n", "Review Tim's pull request".tokenize());
        print!("\n {:?} \n", "Review Tim's pull request +TodoTxtTouch @github".tokenize());
        print!("\n {:?} \n", "(A) Review Tim's pull request +TodoTxtTouch @github".tokenize());
        print!("\n {:?} \n", "(A) 2011-03-01 Review Tim's pull request +TodoTxtTouch @github".tokenize());
        print!("\n {:?} \n", "x Review Tim's pull request".tokenize());
        print!("\n {:?} \n", "x Review Tim's pull request +TodoTxtTouch @github".tokenize());
        print!("\n {:?} \n", "x (A) Review Tim's pull request +TodoTxtTouch @github".tokenize());
        print!("\n {:?} \n", "x (A) 2011-03-02 Review Tim's pull request +TodoTxtTouch @github".tokenize());
        print!("\n {:?} \n", "x (A) 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github".tokenize());
    }

    #[test]
    fn tokens_uncompleted_task_just_description() {
        let tokens = "Review Tim's pull request".tokenize().unwrap();

        assert_eq!(tokens.completed, None);
        assert_eq!(tokens.priority, None);
        assert_eq!(tokens.first_date, None);
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request");
    }

    #[test]
    fn tokens_uncompleted_task_description_with_tags() {
        let tokens = "Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, None);
        assert_eq!(tokens.priority, None);
        assert_eq!(tokens.first_date, None);
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tokens_uncompleted_task_priority_and_description() {
        let tokens = "(A) Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, None);
        assert_eq!(tokens.priority, Some("(A) ".to_string()));
        assert_eq!(tokens.first_date, None);
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tokens_uncompleted_task_full() {
        let tokens = "(A) 2011-03-01 Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, None);
        assert_eq!(tokens.priority, Some("(A) ".to_string()));
        assert_eq!(tokens.first_date, Some("2011-03-01 ".to_string()));
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tokens_completed_task_just_description() {
        let tokens = "x Review Tim's pull request".tokenize().unwrap();

        assert_eq!(tokens.completed, Some("x ".to_string()));
        assert_eq!(tokens.priority, None);
        assert_eq!(tokens.first_date, None);
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request");
    }

    #[test]
    fn tokens_completed_task_description_with_tags() {
        let tokens = "x Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, Some("x ".to_string()));
        assert_eq!(tokens.priority, None);
        assert_eq!(tokens.first_date, None);
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tokens_completed_task_priority_and_description() {
        let tokens = "x (A) Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, Some("x ".to_string()));
        assert_eq!(tokens.priority, Some("(A) ".to_string()));
        assert_eq!(tokens.first_date, None);
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tokens_completed_task_priority_and_descriptionl() {
        let tokens = "x (A) 2011-03-02 Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, Some("x ".to_string()));
        assert_eq!(tokens.priority, Some("(A) ".to_string()));
        assert_eq!(tokens.first_date, Some("2011-03-02 ".to_string()));
        assert_eq!(tokens.second_date, None);
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tokens_completed_task_full() {
        let tokens = "x (A) 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github".tokenize().unwrap();

        assert_eq!(tokens.completed, Some("x ".to_string()));
        assert_eq!(tokens.priority, Some("(A) ".to_string()));
        assert_eq!(tokens.first_date, Some("2011-03-02 ".to_string()));
        assert_eq!(tokens.second_date, Some("2011-03-01 ".to_string()));
        assert_eq!(tokens.description, "Review Tim's pull request +TodoTxtTouch @github");
    }

    #[test]
    fn tag_extractor_should_extract_tags_from_description() {
        let tags = "This @description has a lot of +tags and is due:tomorrow !".extract_tags();

        assert_eq!(tags[0], Tag::Context("description".to_owned()));
        assert_eq!(tags[1], Tag::Project("tags".to_owned()));
        assert_eq!(tags[2], Tag::KeyValue("due:tomorrow".to_owned()));
    }
}
