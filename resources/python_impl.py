"""Python implementation of CSV validation."""

import csv
import itertools
import sys


class Transaction:
    __slots__ = ("account", "txn_id", "amount", "balance")

    def __init__(self, account, txn_id, amount, balance):
        self.account = account
        self.txn_id = txn_id
        self.amount = amount
        self.balance = balance

    @classmethod
    def from_row(cls, row):
        return cls(
            account=row[0],
            txn_id=row[1],
            amount=int(row[2]),
            balance=int(row[3])
        )

    def hash_key(self):
        return "::".join((self.account, self.txn_id))


class PartialTxn:
    __slots__ = ("account", "txn_id", "amount")

    def __init__(self, account, txn_id, amount):
        self.account = account
        self.txn_id = txn_id
        self.amount = amount

    def eq_txn(self, other):
        return (
            self.account == other.account
            and self.txn_id == other.txn_id
            and self.amount == other.amount
        )

    def __repr__(self):
        return f"PartialTxn({self.account}, {self.txn_id}, {self.amount})"


class PartialTxnLookup:
    __slots__ = ("hash_key", "partial_txn")

    def __init__(self, hash_key, partial_txn):
        self.hash_key = hash_key
        self.partial_txn = partial_txn


class TxnSummary:
    __slots__ = ("id", "from_", "to", "amount")

    def __init__(self, id, from_, to, amount):
        self.id = id
        self.from_ = from_
        self.to = to
        self.amount = amount

    @classmethod
    def from_row(cls, row):
        return cls(
            id=row[0],
            from_=row[1],
            to=row[2],
            amount=int(row[3]),
        )

    def partial_txns(self):
        return [
            PartialTxnLookup(
                hash_key="::".join((self.from_, self.id)),
                partial_txn=PartialTxn(
                    account=self.from_,
                    txn_id=self.id,
                    amount=-(self.amount),
                )
            ),
            PartialTxnLookup(
                hash_key="::".join((self.to, self.id)),
                partial_txn=PartialTxn(
                    account=self.to,
                    txn_id=self.id,
                    amount=self.amount,
                )
            ),
        ]


def load_summary(path):
    res = []
    with open(path) as csvfile:
        reader = csv.reader(csvfile)
        for row in itertools.islice(reader, 1, None):
            txn_summary = TxnSummary.from_row(row)
            res.extend(txn_summary.partial_txns())
    return res


def load_transactions(path):
    res = {}
    with open(path) as csvfile:
        reader = csv.reader(csvfile)
        for row in itertools.islice(reader, 1, None):
            txn = Transaction.from_row(row)
            res[txn.hash_key()] = txn
    return res


def validate_summary(summary_items, txns):
    for txn_lookup in summary_items:
        txn = txns.get(txn_lookup.hash_key)
        if txn is None:
            raise RuntimeError(f"Could not find txn: {txn_lookup.partial_txn}")
        if not txn_lookup.partial_txn.eq_txn(txn):
            raise RuntimeError(
                f"Txn mismatch for lookup: {txn_lookup.partial_txn}, txn: {txn}")
    return


def validate(summary_path, txns_path):
    txn_summaries = load_summary(summary_path)
    txns = load_transactions(txns_path)
    validate_summary(txn_summaries, txns)


if __name__ == "__main__":
    validate(sys.argv[1], sys.argv[2])
