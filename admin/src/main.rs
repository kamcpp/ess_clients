use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("simurgh-admin")
        .about("Simurgh Identity Verification System - Admin Client")
        .version("0.1.0")
        .author("Kamran Amini <kam.cpp@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("pkcs12-cert")
                .long("pkcs12-cert")
                .help("The identity PKCS 12 containing the private key and the X509 certificate used while authenticating the admin client program")
                .takes_value(true)
                .required(true)
        )
        .subcommand(
            App::new("add")
                .about("Adds a new entity. Supported entities: [ employee ]")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("employee")
                    .about("Adds a new employee")
                    .args(&[
                        Arg::with_name("nr")
                            .long("nr")
                            .help("Employee's number")
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
                        Arg::with_name("username")
                            .long("username")
                            .help("Employee's username")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("email")
                            .long("email")
                            .help("Employee's email")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("mobile")
                            .long("mobile")
                            .help("Employee's mobile")
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
                        Arg::with_name("id")
                            .long("id")
                            .help("Employee's id to update")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("nr")
                            .long("nr")
                            .help("Employee's number")
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
                        Arg::with_name("username")
                            .long("username")
                            .help("Employee's username")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("email")
                            .long("email")
                            .help("Employee's email")
                            .required(true)
                            .takes_value(true),
                        Arg::with_name("mobile")
                            .long("mobile")
                            .help("Employee's mobile")
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
                        Arg::with_name("id")
                            .long("id")
                            .help("Employee's id to delete")
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
                    .about("List employees")
                    .args(&[
                        Arg::with_name("id")
                            .long("id")
                            .help("Employee's id to list")
                            .takes_value(true),
                    ]),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            match add_matches.subcommand() {
                ("employee", add_employee_matches) => {
                },
                _ => panic!(),
            }
        },
        ("update", Some(update_matches)) => {
            match update_matches.subcommand() {
                ("employee", update_employee_matches) => {
                },
                _ => panic!(),
            }
        },
        ("delete", Some(delete_matches)) => {
            match delete_matches.subcommand() {
                ("employee", delete_employee_matches) => {
                },
                _ => panic!(),
            }
        },
        ("list", Some(list_matches)) => {
            match list_matches.subcommand() {
                ("employee", list_employee_matches) => {
                },
                _ => panic!(),
            }
        },
        _ => panic!(),
    }
}
