use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use parsers::*;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum DescriptionComponent {
    Text(String),
    Project(String),
    Context(String),
    KeyValue(String, String),
}

pub type DescriptionComponents = Vec<DescriptionComponent>;


pub struct ProjectParser {}

impl Parser for ProjectParser {
    type Value = DescriptionComponent;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        lazy_static! {
            static ref PROJECT_REGEX : Regex = Regex::new(r"^\+(?P<project>[^\s]+)").expect("Regex is invalid");
        }

        let captures = PROJECT_REGEX.captures(input)?;
        Some(ParserResult {
            value: DescriptionComponent::Project(String::from(&captures[1])),
            remaining: &input[captures[0].len()..],
        })
    }
}

pub struct ContextParser {}

impl Parser for ContextParser {
    type Value = DescriptionComponent;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        lazy_static! {
            static ref CONTEXT_REGEX : Regex = Regex::new(r"^@(?P<context>[^\s]+)").expect("Regex is invalid");
        }

        let captures = CONTEXT_REGEX.captures(input)?;
        Some(ParserResult {
            value: DescriptionComponent::Context(String::from(&captures[1])),
            remaining: &input[captures[0].len()..],
        })
    }
}

pub struct KeyValueParser {}

impl Parser for KeyValueParser {
    type Value = DescriptionComponent;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        lazy_static! {
            static ref KEY_VALUE_REGEX : Regex = Regex::new(r"^(?P<key>[^\s:]+):(?P<value>[^\s:]+)").expect("Regex is invalid");
        }

        let captures = KEY_VALUE_REGEX.captures(input)?;
        Some(ParserResult {
            value: DescriptionComponent::KeyValue(String::from(&captures[1]), String::from(&captures[2])),
            remaining: &input[captures[0].len()..],
        })
    }
}

pub struct NormalTextParser {}

impl Parser for NormalTextParser {
    type Value = DescriptionComponent;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        lazy_static!{
            static ref NORMAL_TEXT_REGEX : Regex = Regex::new(r"^(?P<text>[^@\+]*?\s|^\s*)(?:(?:@|\+)|[^\s@\+]+:)[^s]").expect("Regex is invalid");
        }

        match NORMAL_TEXT_REGEX.captures(input) {
            Some(captures) => if captures[1].is_empty() {
                    None
                } else {
                    Some(ParserResult {
                        value: DescriptionComponent::Text(String::from(&captures[1])),
                        remaining: &input[captures[1].len()..],
                    })
                },
            None => if input.is_empty() {
                    None
                } else {
                    Some(ParserResult {
                        value: DescriptionComponent::Text(String::from(input)),
                        remaining: "",
                    })
                }
        }
    }
}

pub struct DescriptionComponentParser {}

impl Parser for DescriptionComponentParser {
    type Value = DescriptionComponent;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        ProjectParser::parse(input)
            .or(ContextParser::parse(input))
            .or(KeyValueParser::parse(input))
            .or(NormalTextParser::parse(input))
    }
}

pub struct DescriptionComponentsParser {}

impl Parser for DescriptionComponentsParser {
    type Value = DescriptionComponents;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        let mut description_components = DescriptionComponents::new();
        let mut remaining = input;
        while !remaining.is_empty() {
            if let Some(component) = DescriptionComponentParser::parse(remaining) {
                remaining = component.remaining;
                description_components.push(component.value)
            }
        }
        Some(ParserResult {
            value: description_components,
            remaining: "",
        })
    }
}

pub trait ComponentExtractor {
    fn extract_components(self) -> Vec<DescriptionComponent>;
}

impl<'a> ComponentExtractor for &'a str {
    fn extract_components(self) -> DescriptionComponents {
        DescriptionComponentsParser::parse(self)
            .map_or(DescriptionComponents::new(), |result| result.value)
    }
}

impl Display for DescriptionComponent {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            &DescriptionComponent::Text(ref text) => write!(formatter, "{}", text),
            &DescriptionComponent::Project(ref project) => write!(formatter, "+{}", project),
            &DescriptionComponent::Context(ref context) => write!(formatter, "@{}", context),
            &DescriptionComponent::KeyValue(ref key, ref value) => write!(formatter, "{}:{}", key, value)
        }
    }
}

pub fn description_components_to_string(components: &Vec<DescriptionComponent>) -> String {
    components.iter()
        .map(|ref component| component.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use description_component::*;

    #[test]
    fn project_parser_should_parse_projects() {
        let project_result = ProjectParser::parse("+Project42 Something else").expect("Must parse.");

        assert_eq!(project_result.value, DescriptionComponent::Project(String::from("Project42")));
        assert_eq!(project_result.remaining, " Something else");
    }

    #[test]
    fn project_parser_should_not_parse_non_projects() {
        assert!(ProjectParser::parse("+ Project").is_none());
        assert!(ProjectParser::parse(" +Project").is_none());
    }

    #[test]
    fn context_parser_should_parse_contexts() {
        let context_result = ContextParser::parse("@Context42 Something else").expect("Must parse");

        assert_eq!(context_result.value, DescriptionComponent::Context(String::from("Context42")));
        assert_eq!(context_result.remaining, " Something else");
    }

    #[test]
    fn context_parser_should_not_parse_non_contexts() {
        assert!(ContextParser::parse("@ Context").is_none());
        assert!(ContextParser::parse(" @Context").is_none());
    }

    #[test]
    fn key_value_parser_should_parse_key_values() {
        let key_value_result = KeyValueParser::parse("bla:42 Something else").expect("Must parse");

        assert_eq!(key_value_result.value, DescriptionComponent::KeyValue(String::from("bla"), String::from("42")));
        assert_eq!(key_value_result.remaining, " Something else");
    }

    #[test]
    fn key_value_parser_should_not_parse_non_contexts() {
        assert!(KeyValueParser::parse("key: value").is_none());
        assert!(KeyValueParser::parse("key :value").is_none());
        assert!(KeyValueParser::parse("key : value").is_none());
        assert!(KeyValueParser::parse(" key:value").is_none());
    }

    #[test]
    fn text_parser_should_parse_text_without_tags() {
        let text_result = NormalTextParser::parse("Hello World!").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from("Hello World!")));
        assert_eq!(text_result.remaining, "");
    }

    #[test]
    fn text_parser_should_not_parse_empty_string() {
        assert!(NormalTextParser::parse("").is_none());
    }

    #[test]
    fn text_parser_should_parse_text_before_context() {
        let text_result = NormalTextParser::parse("Text @Context").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from("Text ")));
        assert_eq!(text_result.remaining, "@Context");
    }

    #[test]
    fn text_parser_should_parse_text_before_project() {
        let text_result = NormalTextParser::parse("Text +Project").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from("Text ")));
        assert_eq!(text_result.remaining, "+Project");
    }

    #[test]
    fn text_parser_should_parse_text_before_key_value() {
        let text_result = NormalTextParser::parse("Text key:value").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from("Text ")));
        assert_eq!(text_result.remaining, "key:value");
    }

    #[test]
    fn text_parser_should_parse_space_before_project() {
        let text_result = NormalTextParser::parse(" +Project").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from(" ")));
        assert_eq!(text_result.remaining, "+Project");
    }

    #[test]
    fn text_parser_should_parse_space_before_context() {
        let text_result = NormalTextParser::parse(" @Context").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from(" ")));
        assert_eq!(text_result.remaining, "@Context");
    }

    #[test]
    fn text_parser_should_parse_space_before_key_value() {
        let text_result = NormalTextParser::parse(" key:value").expect("Must parse");

        assert_eq!(text_result.value, DescriptionComponent::Text(String::from(" ")));
        assert_eq!(text_result.remaining, "key:value");
    }

    #[test]
    fn text_parser_should_not_parse_text_when_it_starts_with_a_tag() {
        assert!(NormalTextParser::parse("@Context").is_none());
        assert!(NormalTextParser::parse("+Project").is_none());
        assert!(NormalTextParser::parse("key:value").is_none());
    }

    #[test]
    fn components_extractor_should_extract_components_from_description() {
        let components = "This @description has a lot of +tags and is due:tomorrow !".extract_components();

        // TODO: Parse contexts and projects correctly (preceding space)
        assert_eq!(components[0], DescriptionComponent::Text(String::from("This ")));
        assert_eq!(components[1], DescriptionComponent::Context(String::from("description")));
        assert_eq!(components[2], DescriptionComponent::Text(String::from(" has a lot of ")));
        assert_eq!(components[3], DescriptionComponent::Project(String::from("tags")));
        assert_eq!(components[4], DescriptionComponent::Text(String::from(" and is ")));
        assert_eq!(components[5], DescriptionComponent::KeyValue(String::from("due"), String::from("tomorrow")));
        assert_eq!(components[6], DescriptionComponent::Text(String::from(" !")));
    }

    #[test]
    fn components_can_be_converted_to_string() {
        let description = "This @description has a lot of +tags and is due:tomorrow !";
        let components = description.extract_components();

        assert_eq!(description_components_to_string(&components), description);
    }
}
