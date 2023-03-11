# ix-rs

A trait for values that permit contiguous subranges.  
This is a port of the `Ix` class from Haskell to a trait in Rust.

This crate provides the `Ix` trait:
```rs
pub trait Ix: PartialOrd {
    type RangeIter: Iterator<Item = Self>;
    fn range(min: Self, max: Self) -> Self::RangeIter;
    fn index(min: Self, max: Self, ix: Self) -> usize;
    fn in_range(min: Self, max: Self, ix: Self) -> bool;
    fn range_size(min: Self, max: Self) -> usize;
}
```
