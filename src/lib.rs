#![no_std]

/// A trait for values that permit contiguous subranges.
///
/// Implementations must uphold the following invariants:
/// 1. `in_range(min, max, ix)` if and only if `range(min, max).any(|x| x == ix)`
/// 2. If `in_range(min, max, ix)`, then `range(min, max).nth(index(min, max, ix))` = `ix`
/// 3. `range(min, max).map(|x| index(min, max, x))` yields equal items to `0..range_size(min, max)`
/// 4. `range_size(min, max)` = `range(min, max).count()`
pub trait Ix: PartialOrd {
    /// An iterator over the elements in a range of the implementing type.
    type RangeIter: Iterator<Item = Self>;
    /// Generate an iterator over a range starting from `min` and stopping at `max`.
    /// The resulting iterator must produce `min` and `max` at some point, each.
    fn range(min: Self, max: Self) -> Self::RangeIter;
    /// Get the position of a value inside a range.
    ///
    /// # Panics
    ///
    /// Should panic if the value is not in the range (as determined by [`in_range`]).
    ///
    /// [`in_range`]: Ix::in_range
    fn index(min: Self, max: Self, ix: Self) -> usize;
    /// Check if a given value is inside a range.
    fn in_range(min: Self, max: Self, ix: Self) -> bool;
    /// Get the length of a range.
    fn range_size(min: Self, max: Self) -> usize;
}

mod macros;
pub mod usize_like;
use macros::impl_ix_numeric;

impl_ix_numeric!(u8);
impl_ix_numeric!(u16);
impl_ix_numeric!(u32);
impl_ix_numeric!(u64);
impl_ix_numeric!(u128);
impl_ix_numeric!(i8);
impl_ix_numeric!(i16);
impl_ix_numeric!(i32);
impl_ix_numeric!(i64);
impl_ix_numeric!(i128);
impl_ix_numeric!(usize);
impl_ix_numeric!(isize);
