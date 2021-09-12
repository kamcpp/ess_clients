use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;

use clap::{App, AppSettings, Arg};

use native_tls::{Identity, TlsConnector};

use hyper::{Client, Request, StatusCode, Body};
use hyper::client::HttpConnector;

use hyper_timeout::TimeoutConnector;

use hyper_tls::HttpsConnector;

use tokio::task;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
    pub username: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "secondName")]
    pub second_name: Option<String>,
}

fn create_client(identity_file_path: &str) -> Client<TimeoutConnector<HttpsConnector<HttpConnector>>> {

    // Create identity object
    let mut identity_file = File::open(identity_file_path).unwrap();
    let mut identity_data = vec![];
    identity_file.read_to_end(&mut identity_data).unwrap();
    let identity = Identity::from_pkcs12(&identity_data, "password").unwrap();

    // Create TLS connector
    let tls_connector = TlsConnector::builder()
        .identity(identity)
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

fn main() -> Result<(), Error> {
    let matches = App::new("ess-admin")
        .about("Encryptizer Simurgh System - Admin Client")
        .version("0.1.0")
        .author("Kamran Amini <kam.cpp@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .args(&[
            Arg::with_name("pkcs12-cert")
                .long("pkcs12-cert")
                .help("The identity PKCS-12 containing the private key and the X509 certificate used while authenticating the admin client program")
                .takes_value(true)
                .required(true),
            Arg::with_name("ess-addr")
                .long("ess-addr")
                .help("The address of ESS backend e.g. localhost:30444")
                .takes_value(true)
                .required(true),
        ])
        .subcommand(
            App::new("add")
                .about("Adds a new entity. Supported entities: [ employee ]")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("employee")
                    .about("Adds a new employee")
                    .args(&[
                        Arg::with_name("username")
                            .long("username")
                            .help("Employee's username")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("first-name")
                            .long("first-name")
                            .help("Employee's first name")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("second-name")
                            .long("second-name")
                            .help("Employee's second name")
                            .required(true)
                            .takes_value(true),
                    ]),
                )
        )
        .subcommand(
            App::new("update")
                .about("Updates an entity. Supported entities: [ employee ]")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("employee")
                    .about("Updates an employee")
                    .args(&[
                        Arg::with_name("username")
                            .long("username")
                            .help("Employee's username")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("first-name")
                            .long("first-name")
                            .help("Employee's first name")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("second-name")
                            .long("second-name")
                            .help("Employee's second name")
                            .required(true)
                            .takes_value(true),
                    ]),
                ),
        )
        .subcommand(
            App::new("delete")
                .about("Deletes an entity. Supported entities: [ employee ]")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("employee")
                    .about("Deletes an employee")
                    .args(&[
                        Arg::with_name("username")
                            .long("username")
                            .help("Employee's username")
                            .required(true)
                            .takes_value(true),
                    ]),
                ),
        )
        .subcommand(
            App::new("list")
                .about("Lists an entity. Supported entities: [ employee ]")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("employee")
                    .about("List employees. Provide username to only see the relevant employee record.")
                    .args(&[
                        Arg::with_name("username")
                            .long("username")
                            .help("Employee's username")
                            .required(false)
                            .takes_value(true),
                    ]),
                ),
        )
        .get_matches();

    let mut rt = tokio::runtime::Runtime::new()?;
    let local = task::LocalSet::new();
    local.block_on(&mut rt, async move {

        let ess_addr = matches.value_of("ess-addr").unwrap();
        let client = create_client(matches.value_of("pkcs12-cert").unwrap());

        let req;
        let mut response_has_entites = false;
        match matches.subcommand() {
            ("add", Some(matches)) => {
                match matches.subcommand() {
                    ("employee", Some(matches)) => {
                        let employee = Employee {
                            username: matches.value_of("username").map(|s| s.to_string()),
                            first_name: matches.value_of("first-name").map(|s| s.to_string()),
                            second_name: matches.value_of("second-name").map(|s| s.to_string()),
                        };
                        req = Request::post(format!("https://{}/api/admin/employee", ess_addr))
                                    .header("Content-Type", "application/json")
                                    .body(Body::from(serde_json::to_string(&employee).unwrap())).unwrap();
                    },
                    (entity, _) => panic!("ERROR: Entity is not supported for 'add': {}", entity),
                }
            },
            ("update", Some(matches)) => {
                match matches.subcommand() {
                    ("employee", Some(matches)) => {
                        let username = matches.value_of("username").map(|s| s.to_string());
                        let employee = Employee {
                            username: username.clone(),
                            first_name: matches.value_of("first-name").map(|s| s.to_string()),
                            second_name: matches.value_of("second-name").map(|s| s.to_string()),
                        };
                        req = Request::put(format!("https://{}/api/admin/employee/{}", ess_addr, username.unwrap()))
                                    .header("Content-Type", "application/json")
                                    .body(Body::from(serde_json::to_string(&employee).unwrap())).unwrap();
                    },
                    (entity, _) => panic!("ERROR: Entity is not supported for 'update': {}", entity),
                }
            },
            ("delete", Some(matches)) => {
                match matches.subcommand() {
                    ("employee", Some(matches)) => {
                        let username = matches.value_of("username").map(|s| s.to_string());
                        req = Request::delete(format!("https://{}/api/admin/employee/{}", ess_addr, username.unwrap()))
                                    .body(Body::from("")).unwrap();
                    },
                    (entity, _) => panic!("ERROR: Entity is not supported for 'delete': {}", entity),
                }
            },
            ("list", Some(matches)) => {
                match matches.subcommand() {
                    ("employee", Some(matches)) => {
                        let mut uri = format!("https://{}/api/admin/employee", ess_addr);
                        uri = match matches.value_of("username") {
                            Some(username) => format!("{}/{}", uri, username),
                            None => format!("{}/all", uri),
                        };
                        response_has_entites = true;
                        req = Request::get(uri).body(Body::from("")).unwrap();
                    },
                    (entity, _) => panic!("ERROR: Entity is not supported for 'list': {}", entity),
                }
            },
            (subcommand, _)  => panic!("ERROR: Subcommand is not supported: {}", subcommand),
        }
        match client.request(req).await {
            Ok(resp) => {
                let status = resp.status();
                if resp.status() == StatusCode::OK {
                    match hyper::body::to_bytes(resp).await {
                        Ok(bytes) => {
                            match std::str::from_utf8(bytes.as_ref()) {
                                Ok(body_str) => {
                                    if response_has_entites {
                                        let employees: Result<Vec<Employee>, _> = serde_json::from_str(body_str);
                                        match employees {
                                            Ok(employees) => {
                                                employees.iter().for_each(|employee| println!("{:?}", employee));
                                            },
                                            Err(err) => panic!("ERROR: Error while deserializing the response: {}", err),
                                        }
                                    } else {
                                        println!("{}", body_str);
                                    }
                                },
                                Err(err) => println!("ERROR: Error while converting body to utf8 string: {:?}", err),
                            }
                        },
                        Err(err) => println!("ERROR: Error while reading body: {:?}", err),
                    }
                } else {
                    match hyper::body::to_bytes(resp).await {
                        Ok(bytes) => {
                            match std::str::from_utf8(bytes.as_ref()) {
                                Ok(body_str) => {
                                    println!("ERROR: [{}] {}", status, body_str);
                                },
                                Err(err) => println!("ERROR: Error while converting body to utf8 string: {:?}", err),
                            }
                        },
                        Err(err) => println!("ERROR: Error while reading body: {:?}", err),
                    }
                }
            },
            Err(err) => {
                println!("ERROR: {}", err);
            }
        }
    });
    Ok(())
}
