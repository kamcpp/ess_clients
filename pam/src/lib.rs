use std::os::raw::c_char;
use std::ffi::{CString, CStr};

use chrono::Utc;

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
pub struct NewIdentityVerifyRequestModel {
    pub username: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewIdentityVerifyResponseModel {
    pub reference: String,
    #[serde(rename = "serverUtcDateTime")]
    pub server_utc_dt: i64,
}

#[derive(Debug, Serialize)]
pub struct CheckIdentityVerifyRequestModel {
    pub reference: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}

fn create_client() -> Client<TimeoutConnector<HttpsConnector<HttpConnector>>> {

    // Create TLS connector
    let tls_connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build().unwrap();

    // Create HTTPS connector
    let mut http_connector = HttpConnector::new();
    http_connector.enforce_http(false);
    let https_connector = HttpsConnector::from((http_connector, tls_connector.into()));

    // Create timeout connector
    let mut timeout_connector = TimeoutConnector::new(https_connector);
    timeout_connector.set_connect_timeout(Some(std::time::Duration::from_secs(5)));

    Client::builder().build(timeout_connector)
}

#[no_mangle]
pub extern "C" fn new_identity_verify_request(
    c_simurgh_addr: *const c_char,
    c_username: *const c_char,
    c_reference: *mut c_char,
    c_reference_len: usize
) -> i32 {
    let mut rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(err) => {
            println!("ERROR: Error while creating tokio runtime: {}", err);
            return -1000;
        }
    };
    let local = task::LocalSet::new();
    match local.block_on(&mut rt, async move {
        let simurgh_addr = unsafe { CStr::from_ptr(c_simurgh_addr).to_string_lossy().into_owned() };
        let username = unsafe { CStr::from_ptr(c_username).to_string_lossy().into_owned() };
        let new_id_verify_req = NewIdentityVerifyRequestModel { username: username, client_utc_dt: Utc::now().timestamp() };
        let req = Request::post(format!("https://{}/api/pam/id_verify_req/new", simurgh_addr))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&new_id_verify_req).unwrap())).unwrap();
        let client = create_client();
        let resp = client.request(req).await?;
        let status = resp.status();
        if resp.status() == StatusCode::OK {
            let bytes = hyper::body::to_bytes(resp).await?;
            let body_str =  std::str::from_utf8(bytes.as_ref())?;
            let id_verify_resp: NewIdentityVerifyResponseModel = serde_json::from_str(body_str)?;
            return Ok(id_verify_resp.reference);
        } else {
            return Err(into_err(format!("ERROR: Bad HTTP status '{}'", status)));
        }
    }) {
        Ok(reference) => {
            let reference = CString::new(reference).unwrap();
            let len = std::cmp::min(reference.as_bytes_with_nul().len(), c_reference_len);
            unsafe {
                std::ptr::copy_nonoverlapping(reference.as_ptr(), c_reference, len);
            }
            return 0;
        }
        Err(err) => {
            println!("{}", err);
            return -2000;
        }
    }
}

#[no_mangle]
pub extern "C" fn check_identity_verify_request(
    c_simurgh_addr: *const c_char,
    c_reference: *const c_char,
    c_client_secret: *const c_char,
) -> i32 {
    let mut rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(err) => {
            println!("ERROR: Error while creating tokio runtime: {}", err);
            return -1000;
        }
    };
    let local = task::LocalSet::new();
    match local.block_on(&mut rt, async move {
        let simurgh_addr = unsafe { CStr::from_ptr(c_simurgh_addr).to_string_lossy().into_owned() };
        let reference = unsafe { CStr::from_ptr(c_reference).to_string_lossy().into_owned() };
        let client_secret = unsafe { CStr::from_ptr(c_client_secret).to_string_lossy().into_owned() };
        let check_id_verify_req = CheckIdentityVerifyRequestModel { reference, client_secret, client_utc_dt: Utc::now().timestamp() };
        let req = Request::post(format!("https://{}/api/pam/id_verify_req/check", simurgh_addr))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&check_id_verify_req).unwrap())).unwrap();
        let client = create_client();
        let resp = client.request(req).await?;
        let status = resp.status();
        if resp.status() == StatusCode::OK {
            return Ok(());
        } else {
            return Err(into_err(format!("ERROR: Bad HTTP status '{}'", status)));
        }
    }) {
        Ok(_) => {
            return 0;
        }
        Err(err) => {
            println!("{}", err);
            return -3000;
        }
    }
}
