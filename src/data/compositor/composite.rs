use core::{fmt::Debug, str::FromStr};

use dyn_fmt::AsStrFormatExt;
use num_traits::Saturating;
use thiserror::Error;

use super::Compositor;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("invalid placeholder: \"{0}\": only letters, numbers, hyphens and underscores are allowed")]
    InvalidPlaceholder(String),
    #[error("missing closing brace at \"{0}\", position {1}")]
    MissingClosingBrace(String, usize),
    #[error("missing opening brace at \"{0}\", position {1}")]
    MissingOpeningBrace(String, usize),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Composite {
    fmt: String,
    args_keys: Vec<String>,
}

impl FromStr for Composite {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        Self::parse(str)
    }
}

impl Composite {
    pub(super) fn parse(raw: impl Into<String>) -> Result<Self> {
        let mut pos = 0;
        let raw_str = raw.into();
        let chunks = raw_str.split_inclusive(|c| c == '{' || c == '}').collect::<Vec<&str>>();
        let (fmt, args_keys) = chunks.clone().into_iter().enumerate().try_fold(
            (String::new(), Vec::new()),
            |(mut fmt, mut keys), (i, chunk)| -> Result<(String, Vec<String>)> {
                let prev = if i > 0 { Some(chunks[i - 1]) } else { None };
                let (chunk, len) = parse_chunk(chunk, prev.as_deref(), pos, chunks.len() - i)?;
                pos += len;
                Ok(match chunk {
                    Chunk::Fmt(chunk) => {
                        fmt.push_str(&chunk);
                        (fmt, keys)
                    }
                    Chunk::Placeholder(key) => {
                        keys.push(key);
                        (fmt, keys)
                    }
                })
            },
        )?;

        Ok(Self { fmt, args_keys })
    }

    pub(super) fn format(&self, sub_compositor: &mut impl Compositor<String>) -> String {
        let args = self.args_keys.iter().map(|key| sub_compositor.compose(key.clone())).collect::<Vec<_>>();
        self.fmt.format(&args)
    }
}

fn validate(str: impl Into<String>) -> Result<()> {
    let str = str.into();
    if str
        .clone()
        .into_bytes()
        .iter()
        .any(|c| !c.is_ascii_alphanumeric() && *c != b'-' && *c != b'_')
        || str.is_empty()
    {
        return Err(Error::InvalidPlaceholder(str));
    }

    Ok(())
}

enum Chunk {
    Fmt(String),
    Placeholder(String),
}

fn parse_chunk(chunk: &str, prev: Option<&str>, start: usize, chunks_left: usize) -> Result<(Chunk, usize)> {
    if chunk.ends_with("}") && prev.is_none_or(|p| !p.ends_with("{")) {
        Err(Error::MissingOpeningBrace(chunk.to_string(), start))
    } else if chunk.ends_with("}") && chunk.len() == 1 {
        Err(Error::InvalidPlaceholder("{}".to_string()))
    } else if chunk.ends_with("}") {
        let len = chunk.len();
        let str = &chunk[..len - 1];
        validate(str)?;
        Ok((Chunk::Placeholder(str.to_string()), len))
    } else if chunk.ends_with("{") {
        let len = chunk.len();
        Ok((Chunk::Fmt([&chunk[..len - 1], "{}"].join("")), len))
    } else if prev.is_some_and(|p| p.ends_with("{")) {
        Err(Error::MissingClosingBrace(["{", chunk].join("").to_string(), start - 1))
    } else {
        Ok((Chunk::Fmt(chunk.to_string()), chunk.len()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::plain_string("hello world", "hello world")]
    #[case::placeholder_not_found("hello {not_found}", "hello not_found")]
    #[case::replace_placeholder("hello {ingredient}", "hello potato")]
    #[case::replace_multiple_placeholders("hello {cooking_method} {ingredient}", "hello roast potato")]
    #[case::replace_composite_placeholder("hello {cooked_food}", "hello roast potato")]
    #[case::replace_repeated_placeholders("a {ingredient} is a {ingredient}", "a potato is a potato")]
    #[case::allow_placeholder_with_underscore("hello {topping_1}", "hello salt")]
    #[case::allow_placeholder_with_hyphen("hello {topping-2}", "hello pepper")]
    #[case::allow_placeholder_with_underscore_and_hyphen("hello {topping_3-3}", "hello mayonnaise")]
    #[case::allow_placeholder_that_is_all_underscores_and_hyphens("hello {_-_-_-_-_-}", "hello snake")]
    fn test_parses_and_formats_valid_strings(#[case] input: &str, #[case] expected: &str) {
        let composite = Composite::parse(input.to_string()).expect("Should parse successfully");

        let actual = composite.format(&mut DummySubCompositor::from([
            ("cooking_method", Composite::parse("roast").expect("Should parse successfully")),
            ("ingredient", Composite::parse("potato").expect("Should parse successfully")),
            (
                "cooked_food",
                Composite::parse("{cooking_method} {ingredient}").expect("Should parse successfully"),
            ),
            ("topping_1", Composite::parse("salt").expect("Should parse successfully")),
            ("topping-2", Composite::parse("pepper").expect("Should parse successfully")),
            ("topping_3-3", Composite::parse("mayonnaise").expect("Should parse successfully")),
            ("_-_-_-_-_-", Composite::parse("snake").expect("Should parse successfully")),
        ]));

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::rejects_placeholder_with_symbol("{ingredient$}", Error::InvalidPlaceholder("ingredient$".to_string()))]
    #[case::rejects_placeholder_with_space("{not working}", Error::InvalidPlaceholder("not working".to_string()))]
    #[case::rejects_placeholder_with_empty_string("{}", Error::InvalidPlaceholder("{}".to_string()))]
    #[case::rejects_pair_with_missing_closing_brace("{no_closing_brace", Error::MissingClosingBrace("{no_closing_brace".to_string(), 0))]
    #[case::rejects_pair_with_missing_opening_brace("no_opening_brace}", Error::MissingOpeningBrace("no_opening_brace}".to_string(), 0))]
    fn test_rejects_invalid_placeholders(#[case] input: &str, #[case] expected: Error) {
        let result = Composite::parse(input.to_string()).expect_err("Should have failed to parse");

        assert_eq!(expected, result);
    }

    struct DummySubCompositor {
        map: HashMap<String, Composite>,
    }

    impl<F: IntoIterator<Item = (impl Into<String>, Composite)>> From<F> for DummySubCompositor {
        fn from(value: F) -> Self {
            Self {
                map: value.into_iter().map(|(k, v)| (k.into(), v)).collect(),
            }
        }
    }

    impl Compositor<String> for DummySubCompositor {
        fn compose(&mut self, key: String) -> String {
            self.map
                .get(&key)
                .cloned()
                .unwrap_or(Composite::parse(key).expect("Should parse successfully"))
                .format(self)
        }
    }
}
