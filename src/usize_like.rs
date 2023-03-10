//! Provides `UsizeLike`, a newtype for values that can be converted to and from [`usize`] losslessly.
use core::{iter::Map, ops::RangeInclusive};

use crate::{
    macros::{assert_in_range, assert_ordered},
    Ix,
};

/// A newtype for values that can be converted to and from [`usize`] losslessly.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsizeLike<T>(pub T);

impl<T: Into<usize>> Into<usize> for UsizeLike<T> {
    fn into(self) -> usize {
        self.0.into()
    }
}

impl<T: From<usize>> From<usize> for UsizeLike<T> {
    fn from(value: usize) -> Self {
        UsizeLike(value.into())
    }
}

impl<T: Into<usize> + From<usize> + PartialOrd> Ix for UsizeLike<T> {
    type RangeIter = Map<RangeInclusive<usize>, fn(usize) -> Self>;
    fn range(min: Self, max: Self) -> Self::RangeIter {
        let min: usize = min.into();
        let max: usize = max.into();
        assert_ordered!(min, max);
        (min..=max).map(<UsizeLike<T>>::from)
    }
    fn index(min: Self, max: Self, ix: Self) -> usize {
        let min: usize = min.into();
        let max: usize = max.into();
        let ix: usize = ix.into();
        assert_ordered!(min, max);
        assert_in_range!(min, max, ix);
        ix - min
    }
    fn in_range(min: Self, max: Self, ix: Self) -> bool {
        let min: usize = min.into();
        let max: usize = max.into();
        let ix: usize = ix.into();
        assert_ordered!(min, max);
        min <= ix && ix <= max
    }
    fn range_size(min: Self, max: Self) -> usize {
        let min: usize = min.into();
        let max: usize = max.into();
        assert_ordered!(min, max);
        max - min
    }
}
