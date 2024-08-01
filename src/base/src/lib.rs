// @todo - rust bug
// https://github.com/intellij-rust/intellij-rust/issues/9853
// remove this from time to time to see the actual unused imports
#![allow(unused_imports)]

pub mod auth;
pub mod canister;
pub mod sanitizer;
pub mod types;
pub mod validator;

// prelude
pub(crate) mod prelude {
    pub use candid::CandidType;
    pub use mimic::{
        ic::structures::storable::Bound,
        orm::{
            collections::HashSet,
            traits::{
                Filterable, Orderable, Path, PrimaryKey, Sanitize, Storable, Validate, Visitable,
            },
        },
        types::ErrorVec,
    };
    pub use serde::{Deserialize, Serialize};
}

#[macro_use]
extern crate macros;
extern crate self as base;

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}
