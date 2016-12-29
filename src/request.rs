//! AWS API requests.
//!
//! Wraps the Hyper library to send PUT, POST, DELETE and GET requests.

extern crate lazy_static;

use std::env;
use std::io::{self, Read};
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

use reqwest::Client;
use reqwest::header::{UserAgent, Headers};
use reqwest::StatusCode;
use reqwest::Method;
use reqwest::Error as ReqwestError;

use log::LogLevel::Debug;

use signature::SignedRequest;

// Pulls in the statically generated rustc version.
include!(concat!(env!("OUT_DIR"), "/user_agent_vars.rs"));

// Use a lazy static to cache the default User-Agent header
// because it never changes once it's been computed.
lazy_static! {
    static ref DEFAULT_USER_AGENT: String = format!("rusoto/{} rust/{} {}",
            env!("CARGO_PKG_VERSION"), RUST_VERSION, env::consts::OS);
}

// This had Default as well:
// #[derive(Clone)]
pub struct HttpResponse<T> {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: T
}

impl<T: Read> HttpResponse<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.body.read(buf)
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpDispatchError {
    message: String
}

impl Error for HttpDispatchError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for HttpDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// I'm not sure what this conversion is for.
impl From<ReqwestError> for HttpDispatchError {
    fn from(err: ReqwestError) -> HttpDispatchError {
        HttpDispatchError { message: err.description().to_string() }
    }
}

impl From<IoError> for HttpDispatchError {
    fn from(err: IoError) -> HttpDispatchError {
        HttpDispatchError { message: err.description().to_string() }
    }
}

pub trait DispatchSignedRequest: Read {
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse<Read>, HttpDispatchError>;
}

impl DispatchSignedRequest for Client {
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse<Read>, HttpDispatchError> {
        // TODO: be more graceful for using the builder:
        let hyper_method = match Method::from_str(request.method().as_ref()) {
            Ok(method) => method,
            Err(e) => return Err(HttpDispatchError { message: format!("Unsupported HTTP verb. {}", e) }),
        };
        let mut final_uri = format!("https://{}{}", request.hostname(), request.canonical_path());
        if !request.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", request.canonical_query_string());
        }

        let mut request_builder = self.request(hyper_method.clone(), &final_uri);

        request_builder = match request.payload() {
            None => request_builder,
            Some(payload_contents) => request_builder.body(payload_contents),
        };

        // translate the headers map to a format Hyper likes
        let mut hyper_headers = Headers::new();
        for h in request.headers().iter() {
            hyper_headers.set_raw(h.0.to_owned(), h.1.to_owned());
        }
        request_builder = request_builder.headers(hyper_headers.clone());

        // TODO: overwrite.  Is that the default?
        request_builder = request_builder.header(UserAgent(DEFAULT_USER_AGENT.clone()));

        if log_enabled!(Debug) {
            let payload = request.payload().map(|mut payload_bytes| {
                let mut payload_string = String::new();

                payload_bytes.read_to_string(&mut payload_string)
                    .map(|_| payload_string)
                    .unwrap_or_else(|_| String::from("<non-UTF-8 data>"))
            });

            debug!("Full request: \n method: {}\n final_uri: {}\n payload: {:?}\nHeaders:\n", hyper_method, final_uri, payload);
            for h in hyper_headers.iter() {
                debug!("{}:{}", h.name(), h.value_string());
            }
        }

        let mut hyper_response = try!(request_builder.send());

        let mut body = String::new();
        // We should pass back a Read object instead:
        try!(hyper_response.read_to_string(&mut body));

        if log_enabled!(Debug) {
            debug!("Response body:\n{}", body);
        }

        Ok(HttpResponse {
            status: hyper_response.status().clone(),
            body: hyper_response.response,
            headers: HashMap::new()
        })

    }
}
