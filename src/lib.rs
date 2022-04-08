#![feature(test)]
#![feature(type_alias_impl_trait)]

extern crate test;

pub mod digits;
pub mod duplicates;
pub mod permutations;
pub mod sieve;
pub mod wheel;

#[macro_export]
macro_rules! nzu8 {
    ( $n:expr ) => {
        unsafe { NonZeroU8::new_unchecked($n) }
    };
}
