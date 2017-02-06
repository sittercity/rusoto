#![crate_name = "rusoto_core"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]

/*



extern crate md5;
extern crate regex;

extern crate serde_json;


#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
pub use rusoto_credential::{
    AwsCredentials,
    ChainProvider,
    ContainerProvider,
    CredentialsError,
    EnvironmentProvider,
    InstanceMetadataProvider,
    ProfileProvider,
    ProvideAwsCredentials,
    DefaultCredentialsProvider,
    DefaultCredentialsProviderSync,
};

*/
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate chrono;
extern crate url;
extern crate time;
extern crate ring;
extern crate rusoto_credential;
extern crate rustc_serialize;
extern crate serde;
extern crate xml;
extern crate hyper;
extern crate hyper_native_tls;

pub use region::{ParseRegionError, Region};
pub use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError, TlsError};
pub use signature::SignedRequest;
pub use request::default_tls_client;

pub mod param;
pub mod region;
pub mod request;
pub mod xmlerror;
pub mod xmlutil;
mod serialization;
#[macro_use] pub mod signature;

pub mod mock;

