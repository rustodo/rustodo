use regex::Regex;

#[derive(Debug)]
pub struct TaskTokens {
    pub completed : Option<String>,
    pub priority : Option<String>,
    pub first_date : Option<String>,
    pub second_date : Option<String>,
    pub description : String,
}

pub trait Tokenizer {
    fn tokenize(self) -> Option<TaskTokens>;
}

impl<'a> Tokenizer for &'a str {
    fn tokenize(self) -> Option<TaskTokens> {
        lazy_static! {
            static ref TOKENS_REGEX: Regex = Regex::new(r"^(?P<completed>x )?(?P<priority>\([A-Z]\) )?(?P<first_date>\d{4}-\d{2}-\d{2} )?(?P<second_date>\d{4}-\d{2}-\d{2} )?(?P<description>.*)$").unwrap();
        }

        let captures = match TOKENS_REGEX.captures(self) {
            Some(captures) => captures,
            None => return None
        };

        Some(TaskTokens {
            completed: match captures.name("completed") {
                Some(m) => Some(m.as_str().to_owned()),
                None => None
            },
            priority: match captures.name("priority") {
                Some(m) => Some(m.as_str().to_owned()),
                None => None
            },
            first_date: match captures.name("first_date") {
                Some(m) => Some(m.as_str().to_owned()),
                None => None
            },
            second_date: match captures.name("second_date") {
                Some(m) => Some(m.as_str().to_owned()),
                None => None
            },
            description: match captures.name("description") {
                Some(m) => m.as_str().to_owned(),
                None => return None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use tokens::Tokenizer;

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
}

