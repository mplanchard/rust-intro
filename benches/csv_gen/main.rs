use std::error::Error;

use clap::{App, Arg, ArgMatches};
use csv;
use rand;
use rand::Rng;
use uuid::Uuid;

pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("csv_parser")
        .arg(Arg::with_name("NUM_ACCTS").required(true).index(1))
        .arg(Arg::with_name("TXNS_PER_ACCT").required(true).index(2))
        .get_matches()
}

fn main() -> Result<(), Box<Error>> {
    let num_accts = get_args()
        .value_of("NUM_ACCTS")
        .ok_or("bad num")?
        .parse::<u32>()?;
    let txns_per_acct = get_args()
        .value_of("TXNS_PER_ACCT")
        .ok_or("bad num")?
        .parse::<u32>()?;
    let num_records = num_accts * txns_per_acct;
    let mut rng = rand::thread_rng();
    let mut txn_writer = csv::Writer::from_path(
        format!("resources/txns{}.csv", num_records)
    )?;
    let mut summary_writer = csv::Writer::from_path(
        format!("resources/summary{}.csv", num_records)
    )?;
    txn_writer.write_record(&["account", "txn_id", "amount", "balance"])?;
    summary_writer.write_record(&["id", "from", "to", "amount"])?;
    for _ in 0..num_accts {
        let from_account = Uuid::new_v4().to_string();
        let mut from_balance: i64 = rng.gen_range(0, 100_000);
        for _ in 0..txns_per_acct {
            let txn_id = Uuid::new_v4().to_string();
            let amount: u16 = rng.gen_range(0, 10_000);
            let amount_str = amount.to_string();
            let to_account = Uuid::new_v4().to_string();
            let to_start_balance: usize = rng.gen_range(0, 100_000);
            from_balance -= i64::from(amount);
            summary_writer.write_record(&[&txn_id, &from_account, &to_account, &amount_str])?;
            txn_writer.write_record(&[
                &from_account,
                &txn_id,
                &format!("-{}", amount_str),
                &from_balance.to_string(),
            ])?;
            txn_writer.write_record(&[
                &to_account,
                &txn_id,
                &amount_str,
                &(to_start_balance + usize::from(amount)).to_string()
            ])?;
        }
    }
    Ok(())
}
