use dyn_fmt::AsStrFormatExt;

use super::{Compositor, Result};

#[derive(Clone)]
pub(super) struct Composite {
    fmt: String,
    args_keys: Vec<String>,
}

impl Composite {
    pub(super) fn parse(value: String) -> Result<Self> {
        let mut fmt = String::new();
        let mut args_keys = Vec::new();

        let mut last_end = 0;
        for (start, end) in value.match_indices("{").map(|(i, _)| i).zip(value.match_indices("}").map(|(i, _)| i)) {
            fmt.push_str(&value[last_end..start]);
            fmt.push_str("{}");

            args_keys.push(value[start + 1..end].to_string());
            last_end = end + 1;
        }

        fmt.push_str(&value[last_end..]);

        Ok(Self { fmt, args_keys })
    }

    pub(super) fn format(&self, sub_compositor: &mut impl Compositor<String>) -> String {
        let args = self.args_keys.iter().map(|key| sub_compositor.compose(key.clone())).collect::<Vec<_>>();
        self.fmt.format(&args)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::plain_string("hello world", "hello world")]
    #[case::placeholder_not_found("hello {not_found}", "hello not_found")]
    fn test_parses_and_formats_valid_strings(#[case] input: &str, #[case] expected: &str) {
        let composite = Composite::parse(input.to_string()).expect("Should parse successfully");

        let actual = composite.format(&mut DummySubCompositor);

        assert_eq!(actual, expected);
    }

    struct DummySubCompositor;

    impl Compositor<String> for DummySubCompositor {
        fn compose(&mut self, key: String) -> String {
            key
        }
    }
}
