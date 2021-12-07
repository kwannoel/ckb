use std::collections::HashMap;

use ckb_types::core::TransactionView;
use ckb_types::packed::Byte32; // Should I just use a primitve array?
use std::sync::{Arc, RwLock};

// This is a map between account ids and latest transactions.
pub struct AccountCellMap {
    inner: HashMap<Byte32, TransactionView>
}

impl AccountCellMap {
    pub fn new() -> Self {
        let inner = HashMap::<Byte32, TransactionView>::new();
        Self { inner }
    }
}
