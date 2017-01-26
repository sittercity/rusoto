#![cfg(feature = "sts")]
extern crate rusoto;

use rusoto::sts::{StsClient, GetCallerIdentityRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_get_caller_identity() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = StsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let result = client.get_caller_identity(&GetCallerIdentityRequest);
    println!("{:#?}", result);
}


