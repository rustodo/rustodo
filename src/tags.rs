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
