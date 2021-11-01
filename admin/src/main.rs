use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;

use clap::{App, AppSettings, Arg};

>> ADD DEPENDENCIES HERE <<

use tokio::task;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
    >> FILL HERE <<
}

fn create_client(identity_file_path: &str) -> Client<TimeoutConnector<HttpsConnector<HttpConnector>>> {
    >> FILL HERE <<
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

    >> FILL HERE <<

    Ok(())
}
