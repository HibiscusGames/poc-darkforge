use std::ops::Range;

use super::severity::Severity;

pub trait Capacity {
    const LESSER: usize;
    const MODERATE: usize;
    const SEVERE: usize;
    const FATAL: usize;

    fn capacity(sev: Severity) -> usize {
        match sev {
            Severity::Lesser => Self::LESSER,
            Severity::Moderate => Self::MODERATE,
            Severity::Severe => Self::SEVERE,
            Severity::Fatal => Self::FATAL,
        }
    }

    fn range(sev: Severity) -> Range<usize> {
        let mod_start = Self::LESSER + Self::MODERATE;
        let sev_start = mod_start + Self::SEVERE;
        let fat_start = sev_start + Self::FATAL;

        match sev {
            Severity::Lesser => 0..Self::LESSER,
            Severity::Moderate => Self::LESSER..mod_start,
            Severity::Severe => mod_start..sev_start,
            Severity::Fatal => sev_start..fat_start,
        }
    }

    fn total() -> usize {
        Self::LESSER + Self::MODERATE + Self::SEVERE + Self::FATAL
    }
}

pub struct DefaultCapacity;

impl Capacity for DefaultCapacity {
    const LESSER: usize = 2;
    const MODERATE: usize = 2;
    const SEVERE: usize = 1;
    const FATAL: usize = 1;
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::lesser_defaults_to_2(Severity::Lesser, 2)]
    #[case::moderate_defaults_to_2(Severity::Moderate, 2)]
    #[case::severe_defaults_to_1(Severity::Severe, 1)]
    #[case::fatal_defaults_to_1(Severity::Fatal, 1)]
    fn test_default_severity_capacity_per_level(#[case] sev: Severity, #[case] expected: usize) {
        let got = DefaultCapacity::capacity(sev);

        assert_eq!(expected, got);
    }

    #[rstest]
    #[case::custom_lesser_capacity_is_3(Severity::Lesser, 3)]
    #[case::custom_moderate_capacity_is_3(Severity::Moderate, 3)]
    #[case::custom_severe_capacity_is_2(Severity::Severe, 2)]
    #[case::custom_fatal_capacity_is_2(Severity::Fatal, 2)]
    fn test_custom_severity_capacity_per_level(#[case] sev: Severity, #[case] expected: usize) {
        let got = CustomCapacity::capacity(sev);

        assert_eq!(expected, got);
    }

    #[rstest]
    #[case::lesser_defaults_to_0_to_2(Severity::Lesser, 0..2)]
    #[case::moderate_defaults_to_2_to_4(Severity::Moderate, 2..4)]
    #[case::severe_defaults_to_4_to_5(Severity::Severe, 4..5)]
    #[case::fatal_defaults_to_5_to_6(Severity::Fatal, 5..6)]
    fn test_default_severity_range_per_level(#[case] sev: Severity, #[case] expected: Range<usize>) {
        let got = DefaultCapacity::range(sev);

        assert_eq!(expected, got);
    }

    #[rstest]
    #[case::custom_lesser_capacity_is_0_to_3(Severity::Lesser, 0..3)]
    #[case::custom_moderate_capacity_is_3_to_6(Severity::Moderate, 3..6)]
    #[case::custom_severe_capacity_is_6_to_8(Severity::Severe, 6..8)]
    #[case::custom_fatal_capacity_is_8_to_10(Severity::Fatal, 8..10)]
    fn test_custom_severity_range_per_level(#[case] sev: Severity, #[case] expected: Range<usize>) {
        let got = CustomCapacity::range(sev);

        assert_eq!(expected, got);
    }

    #[test]
    fn test_default_total_capacity() {
        let got = DefaultCapacity::total();

        assert_eq!(6, got);
    }

    #[test]
    fn test_custom_total_capacity() {
        let got = CustomCapacity::total();

        assert_eq!(10, got);
    }

    struct CustomCapacity;
    impl Capacity for CustomCapacity {
        const LESSER: usize = 3;
        const MODERATE: usize = 3;
        const SEVERE: usize = 2;
        const FATAL: usize = 2;
    }
}
