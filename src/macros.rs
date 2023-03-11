macro_rules! assert_ordered {
    ($min: expr, $max: expr) => {
        if $min > $max {
            panic!("min is greater than max");
        }
    };
}

macro_rules! assert_in_range {
    ($min: expr, $max: expr, $ix: expr) => {
        if $min > $ix {
            panic!("ix is outside range (ix < min)");
        } else if $ix > $max {
            panic!("ix is outside range (ix > max)");
        }
    };
}

macro_rules! impl_ix_numeric {
    ($t: ty) => {
        impl $crate::Ix for $t {
            type RangeIter = ::core::ops::RangeInclusive<$t>;
            fn range(min: Self, max: Self) -> Self::RangeIter {
                $crate::macros::assert_ordered!(min, max);
                min..=max
            }
            fn index(min: Self, max: Self, ix: Self) -> usize {
                $crate::macros::assert_ordered!(min, max);
                $crate::macros::assert_in_range!(min, max, ix);
                (ix - min) as usize
            }
            fn in_range(min: Self, max: Self, ix: Self) -> bool {
                $crate::macros::assert_ordered!(min, max);
                min <= ix && ix <= max
            }
            fn range_size(min: Self, max: Self) -> usize {
                $crate::macros::assert_ordered!(min, max);
                (max - min + 1) as usize
            }
        }
    };
}

pub(crate) use assert_in_range;
pub(crate) use assert_ordered;
pub(crate) use impl_ix_numeric;
