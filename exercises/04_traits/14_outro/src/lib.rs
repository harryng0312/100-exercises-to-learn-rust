// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {}

impl From<u16> for SaturatingU16 {
    fn from(s: u16) -> Self {
        SaturatingU16 { value: s }
    }
}
impl From<u8> for SaturatingU16 {
    fn from(s: u8) -> Self {
        SaturatingU16 { value: s as u16 }
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(s: &u16) -> Self {
        SaturatingU16 { value: *s }
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(s: &u8) -> Self {
        SaturatingU16 { value: *s as u16 }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        // self.value.saturating_add(rhs.value).into()
        SaturatingU16::from(self.value.saturating_add(rhs.value))
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16::from(self.value.saturating_add(rhs.value))
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16::from(self.value.saturating_add(rhs))
    }
}
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16::from(self.value.saturating_add(*rhs))
    }
}

impl PartialOrd<SaturatingU16> for SaturatingU16 {
    fn partial_cmp(&self, other: &SaturatingU16) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }

    fn lt(&self, other: &SaturatingU16) -> bool {
        self.value < other.value
    }

    fn le(&self, other: &SaturatingU16) -> bool {
        self.value <= other.value
    }

    fn gt(&self, other: &SaturatingU16) -> bool {
        self.value > other.value
    }

    fn ge(&self, other: &SaturatingU16) -> bool {
        self.value >= other.value
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialOrd<u16> for SaturatingU16 {
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        Some(self.value.cmp(&other))
    }

    fn lt(&self, other: &u16) -> bool {
        self.value < *other
    }

    fn le(&self, other: &u16) -> bool {
        self.value <= *other
    }

    fn gt(&self, other: &u16) -> bool {
        self.value > *other
    }

    fn ge(&self, other: &u16) -> bool {
        self.value >= *other
    }
}