use std::error::Error;
use std::path::Path;
use clap::{App, Arg, ArgMatches};
use csv::{Reader, StringRecord};


trait FromRow {
    fn from_row(row: &StringRecord) -> Result<Box<Self>, &'static str>;
}


#[derive(Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: i64,
}

impl FromRow for Transaction {
    fn from_row(row: &StringRecord) -> Result<Box<Self>, &'static str> {
        Ok(Transaction{
            from: row.get(0).ok_or("no from")?.to_owned(),
            to: row.get(1).ok_or("no to")?.to_owned(),
            amount: row.get(2).ok_or("no amount")?.parse::<i64>().map_err(|_| "bad amount")?,
        }.into())
    }
}


#[derive(Debug)]
struct Balance {
    account: String,
    txn_amount: i64,
    balance: i64,
}
impl FromRow for Balance {
    fn from_row(row: &StringRecord) -> Result<Box<Self>, &'static str> {
        Ok(Balance{
            account: row.get(0).ok_or("no from")?.to_owned(),
            txn_amount: row.get(1).ok_or("no to")?.parse::<i64>().map_err(|_| "bad txn")?,
            balance: row.get(2).ok_or("no amount")?.parse::<i64>().map_err(|_| "bad balance")?,
        }.into())
    }
}


fn load_path<T: FromRow, P: AsRef<Path>>(path: P) -> Result<Vec<T>, Box<Error>> {
    let mut res: Vec<T> = Vec::new();
    let mut reader = Reader::from_path(path)?;
    for row in reader.records() {
        res.push(*T::from_row(&row?)?)
    }
    Ok(res)
}


fn get_args<'a>() -> ArgMatches<'a> {
    App::new("csv_parser")
        .arg(Arg::with_name("TRANSACTIONS").required(true).index(1))
        .arg(Arg::with_name("BALANCES").required(true).index(2))
        .get_matches()
}


fn main() {
    let args = get_args();
    let txns = load_path::<Transaction, &str>(args.value_of("TRANSACTIONS").unwrap());
    let balances = load_path::<Transaction, &str>(args.value_of("BALANCES").unwrap());
    dbg!(txns);
    dbg!(balances);
}
