#![forbid(unsafe_code)]
#![allow(dead_code)]
// #![cfg_attr(feature = "strict", allow(dead_code))]
// #![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate uncover;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

define_uncover_macros!(enable_if(cfg!(debug_assertions)));

// TODO: This is a bit weird that to use the lib you do: chess::chess::...
pub mod chess;
pub mod fixtures;
pub mod test_moves;
