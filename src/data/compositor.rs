use std::{collections::HashMap, hash::Hash};

use rand::prelude::*;

pub trait Compositor<K: Hash + Eq> {
    fn compose(&mut self, selector: K) -> String;
}

pub struct HashMapCompositor<R: Rng, K: Hash + Eq> {
    rng: R,
    map: HashMap<K, Vec<String>>,
}

impl<R: Rng + Sized, K: Hash + Eq> HashMapCompositor<R, K> {
    pub fn new(rng: R) -> Self {
        Self { rng, map: HashMap::new() }
    }

    pub fn put(mut self, key: K, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.map.insert(key, value.into_iter().map(Into::into).collect());
        self
    }
}

impl<K: Hash + Eq> Default for HashMapCompositor<ThreadRng, K> {
    fn default() -> Self {
        Self {
            rng: rand::rng(),
            map: HashMap::new(),
        }
    }
}

impl<R: Rng + Sized, K: Hash + Eq> Compositor<K> for HashMapCompositor<R, K> {
    fn compose(&mut self, key: K) -> String {
        self.map
            .get(&key)
            .and_then(|values| values.choose(&mut self.rng).cloned())
            .unwrap_or_else(|| String::from("No matching description found"))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rand::rngs::mock::StepRng;
    use rstest::rstest;

    use super::*;

    fn create_test_compositor() -> HashMapCompositor<StepRng, String> {
        HashMapCompositor {
            rng: StepRng::new(0, 0),
            map: HashMap::new(),
        }
    }

    #[rstest]
    #[case::depth_of_1(create_test_compositor().put("Key1".to_string(), ["1.1".to_string(), "1.2".to_string(), "1.3".to_string()]).put("Key2".to_string(), ["2.1".to_string(), "2.2".to_string()]), "Key2", "2.1")]
    #[case::depth_of_2(create_test_compositor().put("Key1".to_string(), ["1.1.1".to_string(), "1.1.2".to_string()]).put("Key2".to_string(), ["1.2.1".to_string(), "1.2.2".to_string()]).put("Key3".to_string(), ["2.1.1".to_string(), "2.1.2".to_string()]), "Key2", "1.2.1")]
    fn test_compositor_selects_randomly(
        #[case] mut compositor: HashMapCompositor<StepRng, String>, #[case] selector: String, #[case] expected: String,
    ) {
        let result = compositor.compose(selector);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_compositor_distribution() {
        let mut compositor = HashMapCompositor {
            rng: rand::rng(),
            map: HashMap::new(),
        };

        let key = "test_key".to_string();
        let options = vec!["option1".to_string(), "option2".to_string()];
        compositor.map.insert(key.clone(), options.clone());

        const ITERATIONS: usize = 10000;

        let mut counts = HashMap::new();
        for _ in 0..ITERATIONS {
            let result = compositor.compose(key.clone());
            *counts.entry(result).or_insert(0) += 1;
        }

        assert_eq!(counts.len(), options.len(), "All options should be selected at least once");

        for option in options {
            let count = counts.get(&option).unwrap_or(&0);
            let expected = ITERATIONS / 2;
            let margin = (expected as f64 * 0.05) as usize;

            assert!(
                *count >= expected - margin && *count <= expected + margin,
                "Option {} was selected {} times, expected {} ± {}",
                option,
                count,
                expected,
                margin
            );
        }
    }
}
