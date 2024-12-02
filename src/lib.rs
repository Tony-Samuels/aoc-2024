#![feature(portable_simd, maybe_uninit_array_assume_init)]

use std::{fmt::Debug, hint::unreachable_unchecked};

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;

aoc_lib! { year = 2024 }

trait Assume {
    type T;

    fn assume(self) -> Self::T;
}

impl<T> Assume for Option<T> {
    type T = T;

    fn assume(self) -> Self::T {
        if cfg!(test) {
            self.unwrap()
        } else {
            match self {
                Some(t) => t,
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
}

impl<T, E> Assume for Result<T, E>
where
    E: Debug,
{
    type T = T;

    fn assume(self) -> Self::T {
        if cfg!(test) {
            self.unwrap()
        } else {
            match self {
                Ok(t) => t,
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }
}
