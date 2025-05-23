use std::hash::Hash;

use num_traits::{PrimInt, Signed, Unsigned};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// The value is at the maximum.
    #[error("value clamped to max")]
    ClampedMax,
    /// The value is at the minimum.
    #[error("value clamped to min")]
    ClampedMin,
}

/// A value that can be incremented and decremented and is clamped to a range.
pub trait Value<I: PrimInt + Hash>: Default + Copy + Clone + PartialEq + Eq + Hash {
    /// Increments the action value by the specified amount.
    ///
    /// Returns `Err(ValueError::Max)` if the action value is already at the maximum.
    fn increment(&mut self, amount: I) -> Result<(), Error> {
        self.set(self.get().saturating_add(amount))
    }

    /// Decrements the action value by the specified amount.
    ///
    /// Returns `Err(ValueError::Min)` if the action value is already at the minimum.
    fn decrement(&mut self, amount: I) -> Result<(), Error> {
        // Saturating sub avoids underflow when amount > current.
        self.set(self.get().saturating_sub(amount))
    }

    /// Sets the action value to the specified amount.
    ///
    /// Returns `Err(ValueError::Max)` if the action value is already at the maximum.
    /// Returns `Err(ValueError::Min)` if the action value is already at the minimum.
    fn set(&mut self, amount: I) -> Result<(), Error>;

    /// Returns the current action value.
    fn get(&self) -> I;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnsignedInteger<I: PrimInt + Unsigned + Hash, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> {
    // The minimum value.
    min: I,
    // The maximum value.
    max: I,
    // The current value.
    current: I,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SignedInteger<I: PrimInt + Signed + Hash, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> {
    // The minimum value.
    min: I,
    // The maximum value.
    max: I,
    // The current value.
    current: I,
}

impl<I: PrimInt + Unsigned + Hash, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> Default for UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    fn default() -> Self {
        Self {
            max: I::from(DEFAULT_MAX).expect("DEFAULT_MAX must fit in target type"),
            min: I::from(DEFAULT_MIN).expect("DEFAULT_MIN must fit in target type"),
            current: I::from(DEFAULT_MIN).expect("DEFAULT_MIN must fit in target type"),
        }
    }
}

impl<I: PrimInt + Unsigned + Hash, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    pub fn new(min: I, max: I, current: I) -> Self {
        assert!(DEFAULT_MIN <= DEFAULT_MAX, "DEFAULT_MIN must be <= DEFAULT_MAX");
        assert!(min <= max, "min must be <= max");
        assert!(current >= min && current <= max, "current must be within [min, max]");

        Self { min, max, current }
    }
}

impl<I: PrimInt + Unsigned + Hash, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> Value<I> for UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    fn set(&mut self, amount: I) -> Result<(), Error> {
        let clamped = amount.clamp(self.min, self.max);
        let out = if amount < self.min {
            Err(Error::ClampedMin)
        } else if amount > self.max {
            Err(Error::ClampedMax)
        } else {
            Ok(())
        };
        self.current = clamped;
        out
    }

    fn get(&self) -> I {
        self.current
    }
}

impl<I: PrimInt + Signed + Hash, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> Default for SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    fn default() -> Self {
        Self {
            max: I::from(DEFAULT_MAX).unwrap_or(I::max_value()),
            min: I::from(DEFAULT_MIN).unwrap_or(I::min_value()),
            current: I::from(DEFAULT_MIN).unwrap_or(I::min_value()),
        }
    }
}

impl<I: PrimInt + Signed + Hash, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    pub fn new(min: I, max: I, current: I) -> Self {
        assert!(DEFAULT_MIN <= DEFAULT_MAX, "DEFAULT_MIN must be <= DEFAULT_MAX");
        assert!(min <= max, "min must be <= max");
        assert!(current >= min && current <= max, "current must be within [min, max]");

        Self { min, max, current }
    }
}

impl<I: PrimInt + Signed + Hash, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> Value<I> for SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    fn set(&mut self, amount: I) -> Result<(), Error> {
        let clamped = amount.clamp(self.min, self.max);
        let out = if amount < self.min {
            Err(Error::ClampedMin)
        } else if amount > self.max {
            Err(Error::ClampedMax)
        } else {
            Ok(())
        };
        self.current = clamped;
        out
    }

    fn get(&self) -> I {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;
    use proptest::prelude::*;

    use super::*;

    macro_rules! gen_value_property_tests {
        ($val_type:ident, $bounds_type:ty => $($typ:ty),+) => {
            paste! {
                proptest! {
                    $(
                        #[test]
                        fn [<test_ $typ _value_is_clamped_to_valid_range>](v in $typ::MIN..$typ::MAX) {
                            let mut value = $val_type::<$typ, 10, 100>::default();

                            match value.set(v) {
                                Err(clamped) => match clamped {
                                    $crate::data::value::Error::ClampedMax => assert!(v >= 100 as $typ, "Value clamped when it was lower than max: {v} > 100"),
                                    $crate::data::value::Error::ClampedMin => assert!(v <= 10 as $typ, "Value clamped when it was higher than min: {v} < 10"),
                                },
                                Ok(_) => {
                                    let actual = value.get();

                                    assert!(actual >= 10 as $typ, "Value {actual} must be greater than 10");
                                    assert!(actual <= 100 as $typ, "Value {actual} must be lower than 100");
                                }
                            }
                        }
                    )+
                }

                $(
                    #[test]
                    fn [<test_ $typ _value_cannot_increment_past_max>]() {
                        let mut value = $val_type::<$typ, 10, 100>::default();

                        value.set(95).unwrap();

                        assert!(value.increment(10).is_err());
                    }

                    #[test]
                    fn [<test_ $typ _value_cannot_decrement_past_min>]() {
                        let mut value = $val_type::<$typ, 10, 100>::default();

                        value.set(15).unwrap();

                        assert!(value.decrement(10).is_err());
                    }
                )+
            }
        }
    }

    gen_value_property_tests!(UnsignedInteger, usize => u8, u16, u32, u64, u128, usize);
    gen_value_property_tests!(SignedInteger, isize => i8, i16, i32, i64, i128, isize);
}
