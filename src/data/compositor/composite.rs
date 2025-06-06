use dyn_fmt::AsStrFormatExt;

use super::{Compositor, Result};

#[derive(Clone)]
pub(super) struct Composite {
    fmt: String,
    args_keys: Vec<String>,
}

impl Composite {
    pub(super) fn parse(value: impl Into<String>) -> Result<Self> {
        let value_str = value.into();
        let mut fmt = String::new();
        let mut args_keys = Vec::new();

        let mut last_end = 0;
        for (start, end) in value_str
            .match_indices("{")
            .map(|(i, _)| i)
            .zip(value_str.match_indices("}").map(|(i, _)| i))
        {
            fmt.push_str(&value_str[last_end..start]);
            fmt.push_str("{}");

            args_keys.push(value_str[start + 1..end].to_string());
            last_end = end + 1;
        }

        fmt.push_str(&value_str[last_end..]);

        Ok(Self { fmt, args_keys })
    }

    pub(super) fn format(&self, sub_compositor: &mut impl Compositor<String>) -> String {
        let args = self.args_keys.iter().map(|key| sub_compositor.compose(key.clone())).collect::<Vec<_>>();
        self.fmt.format(&args)
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
    fn test_parses_and_formats_valid_strings(#[case] input: &str, #[case] expected: &str) {
        let composite = Composite::parse(input.to_string()).expect("Should parse successfully");

        let actual = composite.format(&mut DummySubCompositor::from([
            ("cooking_method", Composite::parse("roast").expect("Should parse successfully")),
            ("ingredient", Composite::parse("potato").expect("Should parse successfully")),
            (
                "cooked_food",
                Composite::parse("{cooking_method} {ingredient}").expect("Should parse successfully"),
            ),
        ]));

        assert_eq!(actual, expected);
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
