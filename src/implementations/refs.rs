//! # References Implementation
//!
//! This implements the algorithm more efficiently using references.
//!

use csv;
use csv::{Reader, StringRecord, StringRecordsIter};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
struct Transaction {
    account: String,
    txn_id: String,
    amount: i64,
    balance: i64,
}
impl Transaction {
    fn from_row(row: &StringRecord) -> Result<Self, &'static str> {
        Ok(Transaction {
            account: row.get(0).ok_or("no account")?.to_owned(),
            txn_id: row.get(1).ok_or("no txn_id")?.to_owned(),
            amount: row
                .get(2)
                .ok_or("no to")?
                .parse::<i64>()
                .map_err(|_| "bad txn")?,
            balance: row
                .get(3)
                .ok_or("no amount")?
                .parse::<i64>()
                .map_err(|_| "bad balance")?,
        })
    }

    /// Generate a unique hash key for this transfer.
    ///
    /// Generate a key in such a way such that the key for a given transaction
    /// in the balance sheet is predictable from the transaction summary.
    ///
    /// # Example
    /// ```
    /// # For a transfer of $500 from account A to account B
    /// let txn = TxnSummary{ id: "01", from: "a", to: "b", amount: 50000 }
    ///
    /// # We might find the following transactions in the transaction history
    /// let txn_a_to_b = Transaction { account: "a", txn_id: "01", amount: -50000, balance: 10000 }
    /// let txn_b_to_a = Transaction { account: "b", txn_id: "01", amount: 50000, balance: 10000 }
    ///
    /// # In order to easily find the txn history from the summary item,
    /// # we create a hash based on the account and the txn_id
    ///
    /// assert_eq!(txn_a_to_b.hash_key(), "a::01")
    /// ```
    fn hash_key(&self) -> String {
        [&self.account, "::", &self.txn_id].concat()
    }
}

#[derive(Debug)]
struct PartialTxn<'a> {
    account: &'a str,
    txn_id: &'a str,
    amount: i64,
}
impl<'a> PartialTxn<'a> {
    fn eq_txn(&self, other: &Transaction) -> bool {
        self.account == other.account && self.txn_id == other.txn_id && self.amount == other.amount
    }
}

#[derive(Debug)]
struct PartialTxnLookup<'a> {
    hash_key: String,
    partial_txn: PartialTxn<'a>,
}

#[derive(Debug)]
struct TxnSummary<'a> {
    id: &'a str,
    from: &'a str,
    to: &'a str,
    amount: i64,
}

impl<'a> TxnSummary<'a> {
    fn from_row(row: &'a StringRecord) -> Result<Self, &'static str> {
        Ok(TxnSummary {
            id: row.get(0).ok_or("no from")?,
            from: row.get(1).ok_or("no from")?,
            to: row.get(2).ok_or("no to")?,
            amount: row
                .get(3)
                .ok_or("no amount")?
                .parse::<i64>()
                .map_err(|_| "bad amount")?,
        })
    }
}

impl<'a> TxnSummary<'a> {
    fn partial_txns(&'a self) -> Vec<PartialTxnLookup<'a>> {
        vec![
            PartialTxnLookup {
                hash_key: [&self.from, "::", &self.id].concat(),
                partial_txn: PartialTxn {
                    account: &self.from,
                    txn_id: &self.id,
                    amount: -self.amount,
                },
            },
            PartialTxnLookup {
                hash_key: [&self.to, "::", &self.id].concat(),
                partial_txn: PartialTxn {
                    account: &self.to,
                    txn_id: &self.id,
                    amount: self.amount,
                },
            },
        ]
    }
}

fn load_transactions<P: AsRef<Path>>(
    path: P,
    res: &mut HashMap<String, Transaction>,
) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    for row in reader.records() {
        let txn = Transaction::from_row(&row?)?;
        res.insert(txn.hash_key(), txn);
    }
    Ok(())
}

fn validate_lookup(
    txn_lookup: &PartialTxnLookup,
    txns: &HashMap<String, Transaction>,
) -> Result<&'static str, Box<Error>> {
    match txns.get(&txn_lookup.hash_key) {
        Some(txn) => {
            if !txn_lookup.partial_txn.eq_txn(txn) {
                return Err(format!(
                    "Txn mismatch for lookup: {:?}, txn: {:?}",
                    txn_lookup.partial_txn, txn
                )
                .into());
            }
        }
        None => return Err(format!("Could not find txn: {:?}", txn_lookup.partial_txn).into()),
    }
    Ok("Valid!")
}


use std::thread;
use std::sync::mpsc;
use rayon::prelude::*;


pub fn validate<P1: AsRef<Path>, P2: AsRef<Path>>(
    summary_path: P1,
    txns_path: P2,
) -> Result<&'static str, Box<Error>> {
    let mut txns = HashMap::new();
    load_transactions(txns_path, &mut txns)?;

    let (tx, rx) = mpsc::channel::<Result<&'static str, &'static str>>();

    let mut summary_reader = Reader::from_path(summary_path)?;
    // for row in summary_reader.records() {
    //     // for partial_txn in TxnSummary::from_row(&row?)?.partial_txns().into_par_iter() {
    //     //     validate_lookup(&partial_txn, &txns)?;
    //     // }
    //     TxnSummary::from_row(&row?)?.partial_txns().par_iter()
    //         .map(|partial_txn|

    //         )
    //         validate_lookup(&partial_txn, &txns)?;
    // }
    // summary_reader.records().into_iter().par_iter()
    //     .map(|row| {

    //     })
    // for row in summary_reader.records() {
    //     let row_tx = tx.clone();
    //     thread::spawn(move || {
    //         let txn_summary = TxnSummary::from_row(&row?);
    //         for partial_txn in txn_summary.partial_txns().into_iter() {
    //             if let Err(_) = validate_lookup(&partial_txn, &txns) {
    //                 row_tx.send(Err("err")).unwrap();
    //             } else { row_tx.send(Ok("valid")).unwrap(); };
    //         }
    //     });
    //     // for partial_txn in TxnSummary::from_row(&row?)?.partial_txns().into_iter() {
    //     //     validate_lookup(&partial_txn, &txns)?;
    //     // }
    // }
    Ok("valid")
}
