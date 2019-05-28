use clap::{App, Arg, ArgMatches};
use std::error::Error;

mod implementations;

use implementations::owned;
use implementations::refs;

pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("csv_parser")
        .arg(Arg::with_name("SUMMARY").required(true).index(1))
        .arg(Arg::with_name("TRANSACTIONS").required(true).index(2))
        .arg(
            Arg::with_name("implementation")
                .long("impl")
                .takes_value(true)
                .default_value("owned")
                .possible_values(&["owned", "refs"]),
        )
        .get_matches()
}

fn main() -> Result<(), Box<Error>> {
    let args = get_args();
    let summary_path = args.value_of("SUMMARY").ok_or("No summary path")?;
    let txns_path = args.value_of("TRANSACTIONS").ok_or("No transaction path")?;
    if let Some(implementation) = args.value_of("implementation") {
        match implementation {
            "owned" => return owned::validate(summary_path, txns_path).map(|_| ()),
            "refs" => return refs::validate(summary_path, txns_path).map(|_| ()),
            _ => return Err("Bad implementation".into()),
        }
    };
    Err("Bad arguments".into())
}
