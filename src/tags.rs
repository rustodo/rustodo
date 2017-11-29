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
                tags.push(Tag::Project(project.as_str().to_owned()));
            }

            if let Some(context) = captures.name("context") {
                tags.push(Tag::Context(context.as_str().to_owned()));
            }

            if let (Some(key), Some(value)) = (captures.name("key"), captures.name("value")) {
                tags.push(Tag::KeyValue(key.as_str().to_owned(), value.as_str().to_owned()));
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

        assert_eq!(tags[0], Tag::Context("description".to_owned()));
        assert_eq!(tags[1], Tag::Project("tags".to_owned()));
        assert_eq!(tags[2], Tag::KeyValue("due".to_owned(), "tomorrow".to_owned()));
    }
}
