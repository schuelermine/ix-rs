# ix-rs

A trait for values that permit contiguous subranges.  
This is a port of the `Ix` class from Haskell to a trait in Rust.
However, its use should not be limited to the original design goal.

This crate provides the `Ix` trait:
```rs
pub trait Ix: PartialOrd + Sized {
    type RangeIter: Iterator<Item = Self>;

    // Required methods
    fn range(min: Self, max: Self) -> Self::RangeIter;
    fn index_checked(self, min: Self, max: Self) -> Option<usize>;
    fn in_range(self, min: Self, max: Self) -> bool;
    fn range_size_checked(min: Self, max: Self) -> Option<usize>;

    // Provided methods
    fn index(self, min: Self, max: Self) -> usize { ... }
    fn range_size(min: Self, max: Self) -> usize { ... }
}
```

See the [documentation](https://docs.rs/ix-rs/latest/ix_rs/trait.Ix.html) for more details.
