use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;

#[macro_use] extern crate nickel;
use nickel::Nickel;

extern crate chrono;
use chrono::{DateTime,Local};

extern crate clap;
use clap::{Arg, Command};

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: String, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().
                        append(true).
                        write(true).
                        create(true).
                        open(filename)?;
    match file.write_all(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn log_time(filename: String) -> io::Result<String> {
    let entry = formatted_time_entry();
    {
        let bytes = entry.as_bytes();

        match record_entry_in_log(filename, bytes) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
    }
    Ok(entry)
}

fn do_log_time(logfile_path: String, auth_token: Option<String>) -> String {
    match log_time(logfile_path) {
        Ok(entry) => format!("Entry Logged: {}", entry),
        Err(e) => format!("Error: {}", e)
    }
}

fn main() {
    let matches = Command::new("simple-log")
    .arg(Arg::new("log")
        .short('l')
        .long("logfile")
        .required(true)
        .value_name("LOG FILE")
    )
    .arg(Arg::new("auth")
        .short('t')
        .long("token")
        .value_name("AUTH TOKEN")
    )
    .get_matches(); // builds the instance of ArgMatches



    let logfile_path = matches.get_one::<String>("log").unwrap().to_string();
    let auth_token = match matches.get_one::<String>("auth") {
        Some(str) => Some(str.to_string()),
        None => None
    };
    
    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time(logfile_path.clone(), auth_token.clone())
        }
    });

    
    let _ = server.listen("127.0.0.1:6767");
}
