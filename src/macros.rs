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
            panic!("ix is outside range (self < min)");
        } else if $ix > $max {
            panic!("ix is outside range (self > max)");
        }
    };
}

macro_rules! impl_ix_numeric {
    ($t: ty) => {
        impl $crate::Ix for $t {
            type Range = ::core::ops::RangeInclusive<$t>;
            fn range(min: Self, max: Self) -> Self::Range {
                $crate::macros::assert_ordered!(min, max);
                min..=max
            }
            fn index_checked(self, min: Self, max: Self) -> Option<usize> {
                $crate::macros::assert_ordered!(min, max);
                $crate::macros::assert_in_range!(min, max, self);
                usize::try_from(self - min).ok()
            }
            fn in_range(self, min: Self, max: Self) -> bool {
                $crate::macros::assert_ordered!(min, max);
                min <= self && self <= max
            }
            fn range_size_checked(min: Self, max: Self) -> Option<usize> {
                $crate::macros::assert_ordered!(min, max);
                usize::try_from(max - min)
                    .ok()
                    .and_then(|n| n.checked_add(1))
            }
        }
    };
}

pub(crate) use assert_in_range;
pub(crate) use assert_ordered;
pub(crate) use impl_ix_numeric;
