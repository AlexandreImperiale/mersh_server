#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

/// Definition of an interpreter for mersh exposed commands.
pub mod interpreter;