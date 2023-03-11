use ix_rs::Ix;
use paste::paste;
use proptest::{prop_assert, proptest};

fn ix_uphold_1<T: Ix + Copy>(min: T, max: T, ix: T) -> bool {
    if min > max {
        return true;
    }
    ix.in_range(min, max) == Ix::range(min, max).any(|x| x == ix)
}

fn ix_uphold_2<T: Ix + Copy>(min: T, max: T, ix: T) -> bool {
    if min > max {
        return true;
    }
    if !ix.in_range(min, max) {
        return true;
    }
    Ix::range(min, max).nth(ix.index(min, max)) == Some(ix)
}

fn ix_uphold_3<T: Ix + Copy>(min: T, max: T) -> bool {
    if min > max {
        return true;
    }
    Ix::range(min, max)
        .map(|x| x.index(min, max))
        .eq(0..Ix::range_size(min, max))
}

fn ix_uphold_4<T: Ix + Copy>(min: T, max: T) -> bool {
    if min > max {
        return true;
    }
    Ix::range_size(min, max) == Ix::range(min, max).count()
}

macro_rules! r {
    ($t: ty, 0) => {
        -127..=127
    };
    ($t: ty, 1) => {
        <$t>::MIN..=<$t>::MIN + 127
    };
    ($t: ty, 2) => {
        <$t>::MAX - 127..=<$t>::MAX
    };
}

macro_rules! proptest_ix_uphold_some_numeric {
    ($t: ty, $x: literal) => {
        paste! {
            proptest! {
                #[test]
                fn [<proptest_ix_uphold_1_ $t _ $x>](min in r!($t, $x), max in r!($t, $x), ix in r!($t, $x)) {
                    prop_assert!(ix_uphold_1(min, max, ix))
                }
                #[test]
                fn [<proptest_ix_uphold_2_ $t _ $x>](min in r!($t, $x), max in r!($t, $x), ix in r!($t, $x)) {
                    prop_assert!(ix_uphold_2(min, max, ix))
                }
                #[test]
                fn [<proptest_ix_uphold_3_ $t _ $x>](min in r!($t, $x), max in r!($t, $x)) {
                    prop_assert!(ix_uphold_3(min, max))
                }
                #[test]
                fn [<proptest_ix_uphold_4_ $t _ $x>](min in r!($t, $x), max in r!($t, $x)) {
                    prop_assert!(ix_uphold_4(min, max))
                }
            }
        }
    };
}

macro_rules! proptest_ix_uphold_all_numeric {
    ($t: ty) => {
        proptest_ix_uphold_some_numeric!($t, 0);
        proptest_ix_uphold_some_numeric!($t, 1);
        proptest_ix_uphold_some_numeric!($t, 2);
    };
}

proptest_ix_uphold_all_numeric!(u8);
proptest_ix_uphold_all_numeric!(u16);
proptest_ix_uphold_all_numeric!(u32);
proptest_ix_uphold_all_numeric!(u64);
proptest_ix_uphold_all_numeric!(u128);
proptest_ix_uphold_all_numeric!(usize);
proptest_ix_uphold_all_numeric!(i8);
proptest_ix_uphold_all_numeric!(i16);
proptest_ix_uphold_all_numeric!(i32);
proptest_ix_uphold_all_numeric!(i64);
proptest_ix_uphold_all_numeric!(i128);
proptest_ix_uphold_all_numeric!(isize);
