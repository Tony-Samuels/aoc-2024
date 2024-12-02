#![feature(
    maybe_uninit_array_assume_init,
    maybe_uninit_uninit_array,
    never_type,
    portable_simd
)]

use std::{fmt::Debug, hint::unreachable_unchecked};

use aoc_runner_derive::aoc_lib;

pub mod day1;

aoc_lib! { year = 2024 }

trait Assume: Sized {
    type T;

    fn assume(self) -> Self::T {
        if cfg!(test) {
            self.assume_safe()
        } else {
            self.assume_dangerous()
        }
    }

    fn assume_safe(self) -> Self::T;

    fn assume_dangerous(self) -> Self::T;
}

impl<T> Assume for Option<T> {
    type T = T;

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

    fn assume_safe(self) -> Self::T {
        unreachable!()
    }

    fn assume_dangerous(self) -> Self::T {
        unsafe { unreachable_unchecked() }
    }
}
