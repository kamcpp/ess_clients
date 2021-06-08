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
pub struct VerifyRequestModel {
    pub username: String,
    #[serde(rename = "totpCode")]
    pub totp_code: String,
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
pub extern "C" fn ess_verify(
    c_ess_addr: *const c_char,
    c_username: *const c_char,
    c_totp_code: *const c_char,
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
        let ess_addr = unsafe { CStr::from_ptr(c_ess_addr).to_string_lossy().into_owned() };
        let username = unsafe { CStr::from_ptr(c_username).to_string_lossy().into_owned() };
        let totp_code = unsafe { CStr::from_ptr(c_totp_code).to_string_lossy().into_owned() };
        let verify_req = VerifyRequestModel { username, totp_code };
        let req = Request::post(format!("https://{}/api/pam/verify", ess_addr))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&verify_req).unwrap())).unwrap();
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
            return -1;
        }
    }
}
