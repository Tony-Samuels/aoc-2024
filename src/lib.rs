#![allow(
    clippy::missing_safety_doc,
    clippy::new_without_default,
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
    stmt_expr_attributes,
    unchecked_shifts
)]
use std::{
    cmp::max,
    fmt::Debug,
    hash::Hash,
    hint::unreachable_unchecked,
    intrinsics::{unchecked_add, unchecked_div, unchecked_mul, unchecked_rem, unchecked_shl},
    mem::MaybeUninit,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

aoc_lib! { year = 2024 }

pub const ZERO: u8 = b'0';
pub const EOL: u8 = b'\n';

pub struct BigBitSet<const BYTES: usize>([u8; BYTES]);

impl<const BYTES: usize> BigBitSet<BYTES> {
    pub fn new() -> Self {
        Self([0; BYTES])
    }

    pub unsafe fn calc_byte_mask(&self, index: usize) -> (usize, u8) {
        (
            unchecked_div(index, 8),
            unchecked_shl(1, unchecked_rem(index, 8)),
        )
    }

    pub unsafe fn get_byte_unchecked_mut(&mut self, index: usize) -> &mut u8 {
        self.0.get_unchecked_mut(index)
    }

    pub unsafe fn set_unchecked(&mut self, index: usize) {
        *self.0.get_unchecked_mut(unchecked_div(index, 8)) |=
            unchecked_shl(1, unchecked_rem(index, 8));
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        *self.0.get_unchecked(unchecked_div(index, 8)) & unchecked_shl(1, unchecked_rem(index, 8))
            != 0
    }
}

macro_rules! bit_iter_n {
    ($typ:ty) => {
        paste::paste! {
            pub struct [<BitIter $typ:upper>](pub $typ);

            impl Iterator for [<BitIter $typ:upper>] {
                type Item = usize;

                fn next(&mut self) -> Option<Self::Item> {
                    unsafe fn inner(iter: &mut [<BitIter $typ:upper>]) -> Option<usize> {
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
        }
    };
    ($($typ:ty),*) => {
        $( bit_iter_n! { $typ })*
    };
}

bit_iter_n! { u128, u64 }

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

#[macro_export]
macro_rules! p {
    ($typ:ty, $num:expr) => {
        ($num - b'0') as $typ
    };
    ($typ:ty, $tens:expr, $units:expr) => {
        (($tens - b'0') * 10 + $units - b'0') as $typ
    };
    ($typ:ty, $hundreds:expr, $tens:expr, $units:expr) => {
        (($hundreds - b'0') as $typ * 100 + ($tens - b'0') as $typ * 10 + ($units - b'0') as $typ)
    };
}

pub trait Assume: Sized {
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
pub struct ArrayVec<const N: usize, T> {
    inner: [T; N],
    len: usize,
}

impl<const N: usize, T> Hash for ArrayVec<N, T>
where
    [T]: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.as_slice() }.hash(state)
    }
}

impl<const N: usize, T> ArrayVec<N, T> {
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        &self.inner.get_unchecked(..self.len)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self {
            inner: MaybeUninit::array_assume_init(MaybeUninit::uninit_array()),
            len: 0,
        }
    }
}

impl<const N: usize, T> ArrayVec<N, T>
where
    T: Copy,
{
    #[inline]
    pub unsafe fn push_unchecked(&mut self, item: T) {
        *self.inner.get_unchecked_mut(self.len) = item;
        self.len += 1;
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> T {
        *self.inner.get_unchecked(index)
    }

    #[inline]
    pub unsafe fn pop_unchecked(&mut self) -> T {
        self.len -= 1;
        self.get_unchecked(self.len)
    }
}

impl<const N: usize, T> ArrayVec<N, T>
where
    T: Copy + Default,
    [T; N]:,
{
    pub fn new() -> Self {
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
        unsafe { self.inner.get_unchecked(..self.len).iter() }
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

macro_rules! index_n {
    ($typ:ty) => {
        paste::paste! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub struct [<Index $typ:upper>]<const DIM: usize> {
                y: $typ,
                x: $typ,
            }

            impl<const DIM: usize> [<Index $typ:upper>]<DIM> {
                #[inline]
                pub fn x(x: $typ) -> Self {
                    Self { x, y: 0 }
                }

                #[inline]
                pub fn y(y: $typ) -> Self {
                    Self { x: 0, y }
                }

                #[inline]
                pub unsafe fn checked_to(self) -> Option<usize> {
                    if self.x < 0 || self.y < 0 || self.x >= DIM as _ || self.y >= DIM as _ {
                        debug!("{self:?} is invalid");
                        None
                    } else {
                        Some(unchecked_add(unchecked_mul(self.y as usize, const { DIM + 1 }), self.x as usize))
                    }
                }

                #[inline]
                pub unsafe fn to(self) -> usize {
                    assume!(
                        self.x < DIM as _ && self.y < DIM as _,
                        "{self:?} is too large"
                    );
                    unchecked_add(unchecked_mul(max(self.y, 0) as usize, (const { DIM + 1 })), max(self.x, 0) as usize)
                }

                #[inline]
                pub unsafe fn fro(i: usize) -> Self {
                    Self {
                        y: unchecked_div(i, (DIM + 1)) as _,
                        x: unchecked_rem(i, (DIM + 1)) as _,
                    }
                }
            }

            impl<const DIM: usize> Add for [<Index $typ:upper>]<DIM> {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    unsafe {
                        Self {
                            x: self.x.unchecked_add(rhs.x),
                            y: self.y.unchecked_add(rhs.y),
                        }
                    }
                }
            }

            impl<const DIM: usize> AddAssign for [<Index $typ:upper>]<DIM> {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    *self = *self + rhs;
                }
            }

            impl<const DIM: usize> Sub for [<Index $typ:upper>]<DIM> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    unsafe {
                        Self {
                            x: self.x.unchecked_sub(rhs.x),
                            y: self.y.unchecked_sub(rhs.y),
                        }
                    }
                }
            }

            impl<const DIM: usize> SubAssign for [<Index $typ:upper>]<DIM> {
                fn sub_assign(&mut self, rhs: Self) {
                    *self = *self - rhs;
                }
            }
        }
    };
    ($($typ:ty),*) => {
        $( index_n! { $typ })*
    };
}

index_n! { i8 }
