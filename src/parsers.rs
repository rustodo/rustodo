use chrono::prelude::*;
use regex::*;

pub struct ParserResult<'a, Value> {
    pub value: Value,
    pub remaining: &'a str,
}

pub trait Parser {
    type Value;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>>;
}

pub struct DateParser {}

impl Parser for DateParser {
    type Value = Date<Utc>;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        lazy_static! {
            static ref DATE_REGEX : Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").expect("Regex is invalid");
        }

        let captures = DATE_REGEX.captures(input)?;

        let year_capture = &captures[1];
        let year = i32::from_str_radix(year_capture, 10).ok()?;

        let month_capture = &captures[2];
        let month = u32::from_str_radix(month_capture, 10).ok()?;

        let day_capture = &captures[3];
        let day = u32::from_str_radix(day_capture, 10).ok()?;

        let date = Utc.ymd_opt(year, month, day).latest()?;
        Some(ParserResult::<Self::Value> {
            value: date,
            remaining: &input[captures[0].len()..],
        })
    }
}

pub struct CompletionMarkParser {}

impl Parser for CompletionMarkParser {
    type Value = bool;

    fn parse(input: &str) -> Option<ParserResult<Self::Value>> {
        if input.starts_with("x ") {
            Some(ParserResult {
                value: true,
                remaining: &input[2..],
            })
        } else {
            Some(ParserResult {
                value: false,
                remaining: input,
            })
        }
    }
}


#[cfg(test)]
mod tests {
    use parsers::*;

    #[test]
    fn should_parse_digit_sequence() {
        let date_result = DateParser::parse("2018-11-24and some junk.").expect("Must parse.");

        assert_eq!(date_result.value.year(), 2018);
        assert_eq!(date_result.value.month(), 11);
        assert_eq!(date_result.value.day(), 24);
        assert_eq!(date_result.remaining, "and some junk.");
    }

    #[test]
    fn should_not_parse_with_prefix() {
        let date_result = DateParser::parse("bla2018-12-24");
        assert!(date_result.is_none());
    }

    #[test]
    fn dateparser_should_handle_limits() {
        let date_result = DateParser::parse("9999-99-99");
        assert!(date_result.is_none());
    }

    #[test]
    fn completion_mark_parser_should_parse_completion_marks() {
        let parse_result = CompletionMarkParser::parse("x ").expect("Must parse");

        assert_eq!(parse_result.value, true);
        assert_eq!(parse_result.remaining, "");
    }

    #[test]
    fn completion_mark_parser_should_parse_completion_marks_before_text() {
        let parse_result = CompletionMarkParser::parse("x Some text").expect("Must parse");

        assert_eq!(parse_result.value, true);
        assert_eq!(parse_result.remaining, "Some text");
    }

    #[test]
    fn completion_mark_parse_should_parse_uncompleted() {
        let parse_result = CompletionMarkParser::parse("Some Text").expect("Must parse");

        assert_eq!(parse_result.value, false);
        assert_eq!(parse_result.remaining, "Some Text");
    }

    #[test]
    fn completion_mark_parse_should_parse_with_space() {
        let parse_result = CompletionMarkParser::parse(" Some Text").expect("Must parse");

        assert_eq!(parse_result.value, false);
        assert_eq!(parse_result.remaining, " Some Text");
    }

    #[test]
    fn completion_mark_parse_should_parse_empty_string() {
        let parse_result = CompletionMarkParser::parse("").expect("Must parse");

        assert_eq!(parse_result.value, false);
        assert_eq!(parse_result.remaining, "");
    }
}
