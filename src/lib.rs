#![no_std]

/// A trait for values that permit contiguous subranges.
///
/// Implementations that override the provided functions must ensure
/// their custom implementations are equivalent to the provided ones.
///
/// Implementations must uphold the following invariants:
/// 1. `ix.in_range(min, max)` if and only if `Ix::range(min, max).any(|x| x == ix)`
/// 2. If `ix.in_range(min, max)`, then `Ix::range(min, max).nth(ix.index(min, max)).unwrap() == ix`
/// 3. `Ix::range(min, max).map(|ix| ix.index(min, max))` yields equal items to `0..Ix::range_size(min, max)`
/// 4. `Ix::range_size(min, max)` = `Ix::range(min, max).count()`
pub trait Ix: PartialOrd + Sized {
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
    /// Panics if the resulting index is not representable as a [`usize`].
    /// The default implementation does this by unwrapping the return value of [`index_checked`].
    ///
    /// [`in_range`]: Ix::in_range
    /// [`index_checked`]: Ix::index_checked
    fn index(self, min: Self, max: Self) -> usize {
        self.index_checked(min, max).expect("index too large")
    }
    /// Get the position of a value inside a range.
    /// Checked version of [`index`].
    ///
    /// # Panics
    ///
    /// Should panic if the value is not in the range (as determined by [`in_range`]).
    ///
    /// [`index`]: Ix::index
    /// [`in_range`]: Ix::in_range
    fn index_checked(self, min: Self, max: Self) -> Option<usize>;
    /// Check if a given value is inside a range.
    fn in_range(self, min: Self, max: Self) -> bool;
    /// Get the length of a range.
    ///
    /// # Panics
    ///
    /// Panics if the resulting index is not representable as a [`usize`].
    /// The default implementation does this by unwrapping the return value of [`range_size_checked`].
    ///
    /// [`range_size_checked`]: Ix::range_size_checked
    fn range_size(min: Self, max: Self) -> usize {
        Ix::range_size_checked(min, max).expect("range size too large")
    }
    /// Get the length of a range.
    /// Checked version of [`range_size`].
    ///
    /// [`range_size`]: Ix::range_size
    fn range_size_checked(min: Self, max: Self) -> Option<usize>;
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
