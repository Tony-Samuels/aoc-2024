#![allow(unused)]
#![feature(
    maybe_uninit_array_assume_init,
    maybe_uninit_uninit_array,
    never_type,
    portable_simd,
    stmt_expr_attributes
)]

use std::{fmt::Debug, hint::unreachable_unchecked};

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;

aoc_lib! { year = 2024 }

const DEBUG_ENABLED: bool = cfg!(test) || cfg!(feature = "debug");

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
        let val = $e;
        #[cfg(any(test, feature = "debug"))]
        if !val.as_bool() {
            println!("{}:{}", file!(), line!());
        }

        val.assume()
    }};
    ($e:expr, $($arg:tt)*) => {{
        let val = $e;
        #[cfg(any(test, feature = "debug"))]
        if !val.as_bool() {
            print!("{}:{} ", file!(), line!());
            println!($($arg)*);
        }

        val.assume()
    }};
}

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
