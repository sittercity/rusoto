#![crate_name = "rusoto_sqs"]
#![crate_type = "lib"]

#[cfg(feature = "sqs")]
pub mod sqs;

extern crate hyper;
extern crate rusoto_credential;
extern crate rustc_serialize;
extern crate rusoto;
extern crate xml;
