use std::collections::HashMap;

use ckb_types::core::TransactionView;
use ckb_types::packed::Byte32; // Should I just use a primitve array?
use std::sync::{Arc, RwLock};
use ckb_jsonrpc_types::Script;

pub struct AccountId {
    script: Script,
    id: Vec<u8>,
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
}
