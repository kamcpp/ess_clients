use std::os::raw::c_char;
use std::ffi::{CStr};

use native_tls::TlsConnector;

use hyper::{Client, Request, StatusCode, Body};
use hyper::client::HttpConnector;

use hyper_timeout::TimeoutConnector;

use hyper_tls::HttpsConnector;

use string_error::*;

use tokio::task;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize)]
pub struct VerifyRequestModel {
    >> FILL HERE <<
}

fn create_client() -> Client<TimeoutConnector<HttpsConnector<HttpConnector>>> {
    >> FILL HERE <<
}

#[no_mangle]
pub extern "C" fn ess_verify(
    c_ess_addr: *const c_char,
    c_username: *const c_char,
    c_totp_code: *const c_char,
) -> i32 {
    >> FILL HERE <<
}
