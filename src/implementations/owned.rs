//! # Owned Implementation
//!
//! This implementation of the verification algorithm uses owned
//! values within structs rather than references, in order to avoid
//! the need for lifetimes (for demonstration purposes).
//!

use csv;
use csv::{Reader, StringRecord, StringRecordsIter};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
pub trait FromRow {
    fn from_row(row: &StringRecord) -> Result<Box<Self>, &'static str>;
}

#[derive(Debug)]
pub struct Transaction {
    account: String,
    txn_id: String,
    amount: i64,
    balance: i64,
}
impl FromRow for Transaction {
    fn from_row(row: &StringRecord) -> Result<Box<Self>, &'static str> {
        Ok(Transaction {
            account: row.get(0).ok_or("no from")?.to_owned(),
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
        }
        .into())
    }
}
impl Transaction {
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
pub struct PartialTxn {
    account: String,
    txn_id: String,
    amount: i64,
}
impl PartialTxn {
    fn eq_txn(&self, other: &Transaction) -> bool {
        self.account == other.account && self.txn_id == other.txn_id && self.amount == other.amount
    }
}

#[derive(Debug)]
pub struct PartialTxnLookup {
    hash_key: String,
    partial_txn: PartialTxn,
}

#[derive(Debug)]
pub struct TxnSummary {
    id: String,
    from: String,
    to: String,
    amount: i64,
}

impl FromRow for TxnSummary {
    fn from_row(row: &StringRecord) -> Result<Box<Self>, &'static str> {
        Ok(TxnSummary {
            id: row.get(0).ok_or("no from")?.to_owned(),
            from: row.get(1).ok_or("no from")?.to_owned(),
            to: row.get(2).ok_or("no to")?.to_owned(),
            amount: row
                .get(3)
                .ok_or("no amount")?
                .parse::<i64>()
                .map_err(|_| "bad amount")?,
        }
        .into())
    }
}

impl TxnSummary {
    fn partial_txns(&self) -> Vec<PartialTxnLookup> {
        vec![
            PartialTxnLookup {
                hash_key: [&self.from, "::", &self.id].concat(),
                partial_txn: PartialTxn {
                    account: self.from.clone(),
                    txn_id: self.id.clone(),
                    amount: -self.amount,
                },
            },
            PartialTxnLookup {
                hash_key: [&self.to, "::", &self.id].concat(),
                partial_txn: PartialTxn {
                    account: self.to.clone(),
                    txn_id: self.id.clone(),
                    amount: self.amount,
                },
            },
        ]
    }
}

// struct TxnSummaryRecords<'r>  {
//     records: &'r StringRecordsIter<'r, std::fs::File>
// }
// impl<'r> TxnSummaryRecords<'r> {
//     fn from_row_records (records: &'r StringRecordsIter<'r, std::fs::File>) -> Self {
//         TxnSummaryRecords::<'r> { records }
//     }
// }
// impl<'r> Iterator for TxnSummaryRecords<'r> {
//     type Item = Result<PartialTxnLookup, Box<Error>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let row = self.records.next()?;
//         let result = row
//             .map_err(|e| "Could not parse row")
//             .and_then(|r| TxnSummary::from_row(&r))
//             .map(|txn_summary| txn_summary.partial_txns())
//             .flatten();
//         let result = match row {
//             Ok(r) => TxnSummary,
//             Err(r) => Err(r)
//         }
//     }
// }

pub fn load_summary<P: AsRef<Path>>(path: P) -> Result<Vec<PartialTxnLookup>, Box<Error>> {
    let mut res = Vec::new();
    let mut reader = Reader::from_path(path)?;
    for row in reader.records() {
        TxnSummary::from_row(&row?)?
            .partial_txns()
            .into_iter()
            .for_each(|partial_txn| res.push(partial_txn));
    }
    Ok(res)
}

pub fn load_transactions<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, Transaction>, Box<Error>> {
    let mut res: HashMap<String, Transaction> = HashMap::new();
    let mut reader = Reader::from_path(path)?;
    for row in reader.records() {
        let txn = Transaction::from_row(&row?)?;
        res.insert(txn.hash_key(), *txn);
    }
    Ok(res)
}

pub fn validate_summary(
    summary_items: &Vec<PartialTxnLookup>,
    txns: &HashMap<String, Transaction>,
) -> Result<&'static str, Box<Error>> {
    for txn_lookup in summary_items.iter() {
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
    }
    Ok("Valid!")
}


pub fn validate<P1: AsRef<Path>, P2:AsRef<Path>>(summary_path: P1, txns_path: P2) -> Result<&'static str, Box<Error>> {
    let txn_summaries = load_summary(summary_path)?;
    let txns = load_transactions(txns_path)?;
    validate_summary(&txn_summaries, &txns)
}

