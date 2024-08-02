// @todo - rust bug
// https://github.com/intellij-rust/intellij-rust/issues/9853
// remove this from time to time to see the actual unused imports
#![allow(unused_imports)]

pub mod auth;
pub mod canister;
pub mod sanitizer;
pub mod types;
pub mod validator;

extern crate self as base;

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}
