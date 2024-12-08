#![allow(
    clippy::missing_transmute_annotations,
    incomplete_features,
    internal_features,
    static_mut_refs
)]
#![feature(
    core_intrinsics,
    generic_const_exprs,
    generic_arg_infer,
    inline_const_pat,
    maybe_uninit_array_assume_init,
    maybe_uninit_uninit_array,
    never_type,
    portable_simd,
    stmt_expr_attributes
)]
use std::{fmt::Debug, hash::Hash, hint::unreachable_unchecked};

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

aoc_lib! { year = 2024 }

struct BitIter(u128);

impl Iterator for BitIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
        unsafe fn inner(iter: &mut BitIter) -> Option<usize> {
            if iter.0 == 0 {
                None
            } else {
                let position = iter.0.trailing_zeros() as usize;
                iter.0 &= iter.0.wrapping_sub(1);
                Some(position)
            }
        }

        unsafe { inner(self) }
    }
}

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(any(test, feature = "debug"))]
        {
            println!("{}:{}", file!(), line!());
        }
    };
    ($($arg:tt)*) => {
        #[cfg(any(test, feature = "debug"))]
        {
            print!("{}:{} ", file!(), line!());
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! assume {
    ($e:expr) => {{
        use $crate::Assume as _;

        let val = $e;
        #[cfg(any(test, feature = "debug"))]
        if !val.as_bool() {
            println!("{}:{}", file!(), line!());
        }

        val.assume()
    }};
    ($e:expr, $($arg:tt)*) => {{
        use $crate::Assume as _;

        let val = $e;
        #[cfg(any(test, feature = "debug"))]
        if !val.as_bool() {
            print!("{}:{} ", file!(), line!());
            println!($($arg)*);
        }

        val.assume()
    }};
}

#[allow(unused)]
trait Assume: Sized {
    type T;

    #[cfg(any(test, feature = "debug"))]
    fn as_bool(&self) -> bool;

    fn assume(self) -> Self::T {
        #[cfg(any(test, feature = "debug"))]
        {
            self.assume_safe()
        }
        #[cfg(not(any(test, feature = "debug")))]
        {
            self.assume_dangerous()
        }
    }

    fn assume_safe(self) -> Self::T;

    fn assume_dangerous(self) -> Self::T;
}

impl<T> Assume for Option<T> {
    type T = T;

    #[cfg(any(test, feature = "debug"))]
    fn as_bool(&self) -> bool {
        self.is_some()
    }

    fn assume_safe(self) -> Self::T {
        self.unwrap()
    }

    fn assume_dangerous(self) -> Self::T {
        match self {
            Some(t) => t,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T, E> Assume for Result<T, E>
where
    E: Debug,
{
    type T = T;

    #[cfg(any(test, feature = "debug"))]
    fn as_bool(&self) -> bool {
        self.is_ok()
    }

    fn assume_safe(self) -> Self::T {
        self.unwrap()
    }

    fn assume_dangerous(self) -> Self::T {
        match self {
            Ok(t) => t,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

struct Unreachable;

impl Assume for Unreachable {
    type T = !;

    #[cfg(any(test, feature = "debug"))]
    fn as_bool(&self) -> bool {
        false
    }

    fn assume_safe(self) -> Self::T {
        unreachable!()
    }

    fn assume_dangerous(self) -> Self::T {
        unsafe { unreachable_unchecked() }
    }
}

impl Assume for bool {
    type T = ();

    #[cfg(any(test, feature = "debug"))]
    fn as_bool(&self) -> bool {
        *self
    }

    fn assume_safe(self) -> Self::T {
        assert!(self)
    }

    fn assume_dangerous(self) -> Self::T {
        if !self {
            unsafe { unreachable_unchecked() }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ArrayVec<const N: usize, T> {
    inner: [T; N],
    len: usize,
}

impl<const N: usize, T> Hash for ArrayVec<N, T>
where
    [T]: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<const N: usize, T> ArrayVec<N, T> {
    #[inline]
    fn as_slice(&self) -> &[T] {
        &self.inner[..self.len]
    }

    #[inline]
    fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const N: usize, T> ArrayVec<N, T>
where
    T: Copy,
{
    #[inline]
    unsafe fn push_unchecked(&mut self, item: T) {
        *self.inner.get_unchecked_mut(self.len) = item;
        self.len += 1;
    }

    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        *self.inner.get_unchecked(index)
    }

    #[inline]
    unsafe fn pop_unchecked(&mut self) -> T {
        self.len -= 1;
        self.get_unchecked(self.len)
    }
}

impl<const N: usize, T> ArrayVec<N, T>
where
    T: Copy + Default,
    [T; N]:,
{
    fn new() -> Self {
        Self {
            inner: [T::default(); N],
            len: 0,
        }
    }
}

impl<'a, const N: usize, T> IntoIterator for &'a ArrayVec<N, T> {
    type IntoIter = std::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.inner[..self.len].iter()
    }
}

impl<const N: usize, T> PartialEq for ArrayVec<N, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner[..self.len] == other.inner[..self.len]
    }
}

impl<const N: usize, T> Eq for ArrayVec<N, T> where T: Eq {}
