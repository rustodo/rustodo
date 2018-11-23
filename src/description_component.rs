use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum DescriptionComponent {
    Text(String),
    Project(String),
    Context(String),
    KeyValue(String, String),
}

pub trait ComponentExtractor {
    fn extract_components(self) -> Vec<DescriptionComponent>;
}

impl<'a> ComponentExtractor for &'a str {
    fn extract_components(self) -> Vec<DescriptionComponent> {
        lazy_static! {
            static ref TAG_REGEX : Regex = Regex::new(r"(\+(?P<project>[^\s]+)|@(?P<context>[^\s]+)|(?P<keyvalue>(?P<key>[^\s:]+):(?P<value>[^\s:]+)))").expect("Regex is invalid");
        }

        let mut remaining_description = self;
        let mut components = vec![];
        while !remaining_description.is_empty() {
            match TAG_REGEX.captures(remaining_description) {
                Some(captures) => {
                    let start;
                    let end;
                    let component = if let Some(project) = captures.name("project") {
                        start = project.start() - 1;
                        end = project.end();

                        DescriptionComponent::Project(String::from(project.as_str()))
                    } else if let Some(context) = captures.name("context") {
                        start = context.start() - 1;
                        end = context.end();

                        DescriptionComponent::Context(String::from(context.as_str()))
                    } else if let (Some(key), Some(value)) = (captures.name("key"), captures.name("value")) {
                        start = key.start();
                        end = value.end();

                        DescriptionComponent::KeyValue(String::from(key.as_str()), String::from(value.as_str()))
                    } else {
                        start = remaining_description.len();
                        end = start;

                        DescriptionComponent::Text(String::from(remaining_description))
                    };

                    if (start > 0) && (start != remaining_description.len()) {
                        let text = DescriptionComponent::Text(String::from(&remaining_description[0..start]));
                        components.push(text);
                    }

                    components.push(component);

                    remaining_description = &remaining_description[end..];
                },
                None => {
                    components.push(DescriptionComponent::Text(String::from(remaining_description)));
                    remaining_description = &remaining_description[0..0];
                }
            };
        }

        components
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
    use description_component::ComponentExtractor;
    use description_component::DescriptionComponent;
    use description_component::description_components_to_string;

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
