use std::collections::HashMap;

use ckb_types::core::TransactionView;
use ckb_types::packed::{CellInput, Script}; // Should I just use a primitve array?
use ckb_store::{ChainDB, ChainStore};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AccountId {
    script: Script,
    id: Vec<u8>,
}

impl AccountId {
     pub fn new(script: Script, id: Vec<u8>) -> Self {
        Self { script, id, }
    }
}

// This is a map between account cells and latest transactions.
// It uses the product of the account cell and script to identify txs.
#[derive(Clone)]
pub struct AccountCellMap {
    inner: HashMap<AccountId, TransactionView>,
}

impl AccountCellMap {
    pub fn new() -> Self {
        let inner = HashMap::<AccountId, TransactionView>::new();
        Self { inner }
    }

    pub fn contains_account(&self, account_id: &AccountId) -> bool {
        self.inner.contains_key(account_id)
    }

    pub fn update_account(&mut self, account_id: AccountId, tx: TransactionView) -> Option<TransactionView> {
        self.inner.insert(account_id, tx)
    }

    pub fn get(&self, account_id: &AccountId) -> Option<&TransactionView> {
        self.inner.get(account_id)
    }
}

// NOTE: None here is failure state where we cannot find the account cells as specified by indices.
pub fn extract_account_cells(transaction: &TransactionView, cell_indices: Vec<u8>) -> Option<Vec<CellInput>> {
    let inputs = transaction.inputs();
    let mut res = vec![];
    for idx in cell_indices.iter() {
        let input = inputs.get(usize::from(*idx));
        match input {
            None => { return None },
            Some(input) => { res.push(input) },
        }
    }

    Some(res)
}

