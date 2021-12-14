extern crate alloc;
use alloc::collections::btree_map::BTreeMap;

use std::collections::HashMap;

use ckb_types::packed::Transaction;
// use ckb_types::core::TransactionView;
use ckb_types::packed::{CellInput, Script}; // Should I just use a primitve array?
use ckb_store::{ChainDB, ChainStore};

use rebase_auction::AvoumKey;

// This is a map between account cells and latest transactions.
// It uses the product of the account cell and script to identify txs.
#[derive(Clone)]
pub struct AccountCellMap {
    inner: BTreeMap<AvoumKey, Transaction>,
}

impl AccountCellMap {
    pub fn new() -> Self {
        let inner = BTreeMap::<AvoumKey, Transaction>::new();
        Self { inner }
    }

    pub fn contains_account(&self, account_id: &AvoumKey) -> bool {
        self.inner.contains_key(account_id)
    }

    pub fn update_account(&mut self, account_id: AvoumKey, tx: Transaction) -> Option<Transaction> {
        self.inner.insert(account_id, tx)
    }

    pub fn get(&self, account_id: &AvoumKey) -> Option<&Transaction> {
        self.inner.get(account_id)
    }
}
