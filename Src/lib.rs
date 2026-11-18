// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod context;
pub mod hist;
pub mod lex;
pub mod parse;
pub mod signal;
pub mod utils;

pub type zlong = i64;
pub type zulong = u64;

#[unsafe(no_mangle)]
pub const ZLONG_MAX: zlong = i64::MAX;
