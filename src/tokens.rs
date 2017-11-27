use regex::Regex;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Tag {
    Project(String),
    Context(String),
    KeyValue(String),
}

pub trait TagExtractor {
    fn extract_tags(self) -> Vec<Tag>;
}

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

impl<'a> TagExtractor for &'a str {
    fn extract_tags(self) -> Vec<Tag> {
        lazy_static! {
            static ref TAG_REGEX : Regex = Regex::new(r"(\+(?P<project>[^\s]+)|@(?P<context>[^\s]+)|(?P<keyvalue>[^\s:]+:[^\s:]+))").unwrap();
        }

        let mut tags = vec![];
        for captures in TAG_REGEX.captures_iter(self) {
            if let Some(project) = captures.name("project") {
                tags.push(Tag::Project(project.as_str().to_owned()));
            }

            if let Some(context) = captures.name("context") {
                tags.push(Tag::Context(context.as_str().to_owned()));
            }

            if let Some(keyvalue) = captures.name("keyvalue") {
                tags.push(Tag::KeyValue(keyvalue.as_str().to_owned()));
            }
        }

        tags
    }
}
