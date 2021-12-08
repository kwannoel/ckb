use std::collections::HashMap;

use ckb_types::core::TransactionView;
use ckb_types::packed::Script; // Should I just use a primitve array?

#[derive(Eq, PartialEq, Hash)]
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
}
