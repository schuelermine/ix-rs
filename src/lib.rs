#![no_std]
//! This crate provides a trait ([`Ix`]) for values that permit contiguous subranges.  

/// A trait for values that permit contiguous subranges.
///
/// Implementations that override the provided functions must ensure
/// their custom implementations are equivalent to the provided ones.
///
/// Implementations must uphold the following invariants:
/// 1. `ix.in_range(min, max)` if and only if `Ix::range(min, max).any(|x| x == ix)`
/// 2. If `ix.in_range(min, max)`, then `Ix::range(min, max).nth(ix.index(min, max)) == Some(ix)`
/// 3. `Ix::range(min, max).map(|x| x.index(min, max))` yields equal items to `0..Ix::range_size(min, max)`
/// 4. `Ix::range(min, max).map(|x| x.index_checked(min, max))` ever yields [`None`] if and only if `Ix::range_size_checked(min, max).is_none()`
/// 5. `Ix::range_size(min, max) == Ix::range(min, max).count()`
/// 6. `Ix::range_size_checked(min, max).is_none()` if and only if `Ix::range(min, max).count()` overflows or panics
///
/// Note that, for these properties, if one side of the equality panics or overflows the equality can be considered to hold.
///
/// # Examples
///
/// ```
/// # use ix_rs::Ix;
/// for (ix, i) in Ix::range(-45i128, 483).zip(0..) {
///     assert_eq!(ix.index(-45, 483), i);
/// }
/// ```
/// ```
/// # use ix_rs::Ix;
/// assert!(!(-30289i16).in_range(-746, 15564));
/// ```
/// ```
/// # use ix_rs::Ix;
/// assert_eq!(
///     2410117514u32.in_range(2073922791, 3401563124),
///     Ix::range(2073922791u32, 3401563124).any(|x| x == 2410117514)
/// ); // Property 1
/// ```
/// ```
/// # use ix_rs::Ix;
/// assert!(20i32.in_range(17, 5432));
/// assert_eq!(Ix::range(17i32, 5432).nth(20i32.index(17, 5432)).unwrap(), 20);
/// // Property 2
/// ```
/// ```
/// # use ix_rs::Ix;
/// assert!(0.in_range(-31597i16, 16417));
/// assert_eq!(Ix::range(-31597i16, 16417).nth(0.index(-31597i16, 16417)).unwrap(), 0);
/// // Property 2
/// ```
/// ```
/// # use ix_rs::Ix;
/// assert!(
///     Ix::range(-633i32, 151)
///         .map(|x| x.index(-633, 151))
///         .eq(0..Ix::range_size(-633, 151))
/// ) // Property 3
/// ```
/// ```
/// # use ix_rs::Ix;
/// assert_eq!(Ix::range(8079u32, 1836091).count(), Ix::range_size(8079u32, 1836091))
/// // Property 5
/// ```
pub trait Ix: PartialOrd + Sized {
    /// An iterator over the elements in a range of the implementing type.
    type Range: Iterator<Item = Self>;
    /// Generate an iterator over a range starting from `min` and stopping at `max`.
    /// The resulting iterator must produce `min` and `max` at some point, each.
    ///
    /// # Panics
    ///
    /// Should panic if `min` is greater than `max`.
    fn range(min: Self, max: Self) -> Self::Range;
    /// Get the position of a value inside a range.
    ///
    /// # Panics
    ///
    /// Should panic if `min` is greater than `max`.
    ///
    /// Should panic if the value is not in the range (as determined by [`in_range`]).
    ///
    /// Panics if the resulting index is not representable as a [`usize`] value.
    /// The default implementation does this by unwrapping the return value of [`index_checked`].
    ///
    /// [`in_range`]: Ix::in_range
    /// [`index_checked`]: Ix::index_checked
    fn index(self, min: Self, max: Self) -> usize {
        self.index_checked(min, max).expect("index too large")
    }
    /// Get the position of a value inside a range.
    /// If this would overflow the range of [`usize`], returns [`None`].
    /// Checked version of [`index`].
    ///
    /// # Panics
    ///
    /// Should panic if `min` is greater than `max`.
    ///
    /// Should panic if the value is not in the range (as determined by [`in_range`]).
    ///
    /// [`index`]: Ix::index
    /// [`in_range`]: Ix::in_range
    fn index_checked(self, min: Self, max: Self) -> Option<usize>;
    /// Check if a given value is inside a range.
    ///
    /// # Panics
    ///
    /// Should panic if `min` is greater than `max`.
    fn in_range(self, min: Self, max: Self) -> bool;
    /// Get the length of a range.
    ///
    /// # Panics
    ///
    /// Should panic if `min` is greater than `max`.
    ///
    /// Panics if the resulting size is not representable as a [`usize`] value.
    /// The default implementation does this by unwrapping the return value of [`range_size_checked`].
    ///
    /// [`range_size_checked`]: Ix::range_size_checked
    fn range_size(min: Self, max: Self) -> usize {
        Ix::range_size_checked(min, max).expect("range size too large")
    }
    /// Get the length of a range.
    /// If this would overflow the range of [`usize`], returns [`None`].
    /// Checked version of [`range_size`].
    ///
    /// # Panics
    ///
    /// Should panic if `min` is greater than `max`.
    ///
    /// [`range_size`]: Ix::range_size
    fn range_size_checked(min: Self, max: Self) -> Option<usize>;
}

macro_rules! assert_ordered {
    ($min: expr, $max: expr) => {
        if $min > $max {
            panic!("min is greater than max");
        }
    };
}

macro_rules! assert_in_range {
    ($min: expr, $max: expr, $ix: expr) => {
        if $ix < $min {
            panic!("index is outside range (< min)");
        } else if $ix > $max {
            panic!("index is outside range (> max)");
        }
    };
}

macro_rules! impl_ix_numeric {
    ($($t: ty),+) => {
        $(
            impl $crate::Ix for $t {
                type Range = ::core::ops::RangeInclusive<$t>;
                fn range(min: Self, max: Self) -> Self::Range {
                    assert_ordered!(min, max);
                    min..=max
                }
                fn index_checked(self, min: Self, max: Self) -> Option<usize> {
                    assert_ordered!(min, max);
                    assert_in_range!(min, max, self);
                    usize::try_from(self - min).ok()
                }
                fn in_range(self, min: Self, max: Self) -> bool {
                    assert_ordered!(min, max);
                    min <= self && self <= max
                }
                fn range_size_checked(min: Self, max: Self) -> Option<usize> {
                    assert_ordered!(min, max);
                    usize::try_from(max - min)
                        .ok()
                        .and_then(|n| n.checked_add(1))
                }
            }
        )+
    };
}

impl_ix_numeric!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
