use regex::Regex;

enum TagType {
    Project,
    Context,
    KeyValue,
}

struct Tag {
    pub tag_type : TagType,
    pub tag : String,
}

trait TagExtractor {
    fn extract_tags(description : &str) -> Vec<Tag>;
}

pub trait Tokenizer {
    fn tokenize(self) -> Option<TaskTokens>;
}

#[derive(Debug)]
pub struct TaskTokens {
    pub completed : Option<String>,
    pub priority : Option<String>,
    pub first_date : Option<String>,
    pub second_date : Option<String>,
    pub description : String,
}

impl<'a> Tokenizer for &'a str {
    fn tokenize(self) -> Option<TaskTokens> {
        let full_regex = Regex::new(r"^(?P<completed>x )?(?P<priority>\([A-Z]\) )?(?P<first_date>\d{4}-\d{2}-\d{2} )?(?P<second_date>\d{4}-\d{2}-\d{2} )?(?P<description>.*)$").unwrap();

        let captures = match full_regex.captures(self) {
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
