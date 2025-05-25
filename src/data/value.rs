use std::{
    any::type_name,
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use num_traits::{PrimInt, Signed, Unsigned};
use thiserror::Error;

use super::{Result, Value};

#[derive(Error, Debug)]
pub enum Error {
    /// The value is at the maximum.
    #[error("value clamped to max")]
    ClampedMax,
    /// The value is at the minimum.
    #[error("value clamped to min")]
    ClampedMin,
    /// The value is out of bounds.
    #[error("value {0} must be between {1} and {2}")]
    OutOfBounds(String, String, String),
    /// InvalidBounds
    #[error("invalid bounds: {0} must be less than {1}")]
    InvalidBounds(String, String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnsignedInteger<I: PrimInt + Unsigned + Hash, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize>(Integer<I>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SignedInteger<I: PrimInt + Signed + Hash, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize>(Integer<I>);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Integer<I: PrimInt + Hash> {
    // The minimum value.
    min: I,
    // The maximum value.
    max: I,
    // The current value.
    current: I,
}

impl<I: PrimInt + Unsigned + Hash + Debug, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    pub fn new(min: I, max: I, current: I) -> Result<Self> {
        assert!(
            DEFAULT_MIN <= DEFAULT_MAX,
            "DEFAULT_MIN ({DEFAULT_MIN}) must be <= DEFAULT_MAX ({DEFAULT_MAX})"
        );
        Ok(Self(Integer::new(min, max, current)?))
    }
}

impl<I: PrimInt + Unsigned + Hash + Debug + Default, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> Default
    for UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX>
{
    fn default() -> Self {
        let min = I::from(DEFAULT_MIN).unwrap_or_else(|| panic!("DEFAULT_MIN ({DEFAULT_MIN}) could not be cast to {:?}", type_name::<I>()));
        let max = I::from(DEFAULT_MAX).unwrap_or_else(|| panic!("DEFAULT_MAX ({DEFAULT_MAX}) could not be cast to {:?}", type_name::<I>()));

        Self(Integer::new(min, max, I::from(DEFAULT_MIN).unwrap_or_default()).unwrap())
    }
}

impl<I: PrimInt + Unsigned + Hash + Debug, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> Deref
    for UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX>
{
    type Target = Integer<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: PrimInt + Unsigned + Hash + Debug, const DEFAULT_MIN: usize, const DEFAULT_MAX: usize> DerefMut
    for UnsignedInteger<I, DEFAULT_MIN, DEFAULT_MAX>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I: PrimInt + Signed + Hash + Debug, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    pub fn new(min: I, max: I, current: I) -> Result<Self> {
        assert!(DEFAULT_MIN <= DEFAULT_MAX, "DEFAULT_MIN must be <= DEFAULT_MAX");
        Ok(Self(Integer::new(min, max, current)?))
    }
}

impl<I: PrimInt + Signed + Hash + Debug + Default, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> Default
    for SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX>
{
    fn default() -> Self {
        let min = I::from(DEFAULT_MIN).unwrap_or_else(|| panic!("DEFAULT_MIN ({DEFAULT_MIN}) could not be cast to {:?}", type_name::<I>()));
        let max = I::from(DEFAULT_MAX).unwrap_or_else(|| panic!("DEFAULT_MAX ({DEFAULT_MAX}) could not be cast to {:?}", type_name::<I>()));

        Self(Integer::new(min, max, I::from(DEFAULT_MIN).unwrap_or_default()).unwrap())
    }
}

impl<I: PrimInt + Signed + Hash + Debug, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> Deref for SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    type Target = Integer<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: PrimInt + Signed + Hash + Debug, const DEFAULT_MIN: isize, const DEFAULT_MAX: isize> DerefMut for SignedInteger<I, DEFAULT_MIN, DEFAULT_MAX> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I: PrimInt + Hash + Debug> Integer<I> {
    pub fn new(min: I, max: I, current: I) -> Result<Self> {
        if min > max {
            return Err(Error::InvalidBounds(format!("{:?}", min), format!("{:?}", max)).into());
        }
        if current < min || current > max {
            return Err(Error::OutOfBounds(format!("{:?}", current), format!("{:?}", min), format!("{:?}", max)).into());
        }

        Ok(Self { min, max, current })
    }
}

impl<I: PrimInt + Hash + Debug + Default> Value<I> for Integer<I> {
    fn increment(&mut self, amount: I) -> Result<I> {
        let target = self.get().saturating_add(amount);
        if target >= self.max {
            self.current = self.max;
            return Err(Error::ClampedMax.into());
        }

        self.current = target;
        Ok(target)
    }

    fn decrement(&mut self, amount: I) -> Result<I> {
        let target = self.get().saturating_sub(amount);
        if target <= self.min {
            self.current = self.min;
            return Err(Error::ClampedMin.into());
        }

        self.current = target;
        Ok(target)
    }

    fn set(&mut self, amount: I) -> Result<I> {
        if amount < self.min {
            self.current = self.min;
            return Err(Error::ClampedMin.into());
        }
        if amount > self.max {
            self.current = self.max;
            return Err(Error::ClampedMax.into());
        }

        self.current = amount;
        Ok(amount)
    }

    fn get(&self) -> I {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use paste2::paste;
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
                                Err(e) => match e {
                                    $crate::data::Error::Value($crate::data::value::Error::ClampedMax) => assert!(v >= 100 as $typ, "Value clamped when it was lower than max: {v} > 100"),
                                    $crate::data::Error::Value($crate::data::value::Error::ClampedMin) => assert!(v <= 10 as $typ, "Value clamped when it was higher than min: {v} < 10"),
                                    _ => panic!("unexpected error: {}", e)
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
