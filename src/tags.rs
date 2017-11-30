use regex::Regex;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Tag {
    Project(String),
    Context(String),
    KeyValue(String, String),
}

pub trait TagExtractor {
    fn extract_tags(self) -> Vec<Tag>;
}

impl<'a> TagExtractor for &'a str {
    fn extract_tags(self) -> Vec<Tag> {
        lazy_static! {
            static ref TAG_REGEX : Regex = Regex::new(r"(\+(?P<project>[^\s]+)|@(?P<context>[^\s]+)|(?P<keyvalue>(?P<key>[^\s:]+):(?P<value>[^\s:]+)))").unwrap();
        }

        let mut tags = vec![];
        for captures in TAG_REGEX.captures_iter(self) {
            if let Some(project) = captures.name("project") {
                tags.push(Tag::Project(String::from(project.as_str())));
            }

            if let Some(context) = captures.name("context") {
                tags.push(Tag::Context(String::from(context.as_str())));
            }

            if let (Some(key), Some(value)) = (captures.name("key"), captures.name("value")) {
                tags.push(Tag::KeyValue(String::from(key.as_str()), String::from(value.as_str())));
            }
        }

        tags
    }
}

#[cfg(test)]
mod tests {
    use tags::TagExtractor;
    use tags::Tag;

    #[test]
    fn tag_extractor_should_extract_tags_from_description() {
        let tags = "This @description has a lot of +tags and is due:tomorrow !".extract_tags();

        assert_eq!(tags[0], Tag::Context(String::from("description")));
        assert_eq!(tags[1], Tag::Project(String::from("tags")));
        assert_eq!(tags[2], Tag::KeyValue(String::from("due"), String::from("tomorrow")));
    }
}
