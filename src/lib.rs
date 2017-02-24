#![cfg_attr(feature = "unstable", feature(const_fn, drop_types_in_const))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]



extern crate inflector;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate rusoto_core;

#[macro_use]
extern crate serde_derive;

// use std::fs::{File, rename};
// use std::io::BufReader;
// use std::path::Path;

//use botocore::Service as BotocoreService;
//use generator::generate_source;

pub mod botocore;
pub mod generator;
mod serialization;
mod util;
mod acm;