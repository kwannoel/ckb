extern crate alloc;
use alloc::collections::btree_map::BTreeMap;
use ckb_types::packed::{Bytes, CellOutput, Transaction};
use ckb_types::prelude::Unpack;
use ckb_standalone_types::packed as standalone;

use rebase_auction::{AvoumKey, RebaseError};
use auction_utils::types::AuctionState;

// This is a map between account cells and latest transactions.
// It uses the product of the account cell and script to identify txs.
#[derive(Clone)]
pub struct AccountCellMap {
    inner: BTreeMap<AvoumKey, standalone::Transaction>,
}

impl AccountCellMap {
    pub fn new() -> Self {
        let inner = BTreeMap::<AvoumKey, standalone::Transaction>::new();
        Self { inner }
    }

    pub fn contains_account(&self, account_id: &AvoumKey) -> bool {
        self.inner.contains_key(account_id)
    }

    pub fn update_account(&mut self, account_id: AvoumKey, tx: Transaction) -> Option<Transaction> {
        let tx = cast::standalone_transaction(tx);
        let res = self.inner.insert(account_id, tx);
        res.map(cast::core_types_transaction)
    }

    pub fn get(&self, account_id: &AvoumKey) -> Option<&Transaction> {
        self.inner.get(account_id).map(cast::core_types_transaction_ref)
    }

    pub fn inner(&self) -> &BTreeMap<AvoumKey, standalone::Transaction> {
        &self.inner
    }
}

/// Decodes account cell with its parameters and data.
pub fn decode_account_cell(output_data: &Bytes) -> Option<AuctionState> {
    let cell_data: Vec<u8> = output_data.unpack();
    let auction_state = auction_utils::decode_slice::<AuctionState>(&cell_data).ok()?; // TODO: Propagate the error / log it.
    Some(auction_state)
}

/// Reconstructs account key
pub fn make_account_key(cell_output: &CellOutput, output_data: &Bytes) -> Option<AvoumKey> {
    let auction_state = decode_account_cell(output_data)?;
    let type_script = cell_output.type_().to_opt()?;
    // TODO: Generalize account id encoding to first 32 bytes.
    let account_id = auction_state.avoum_id;
    let account_key = AvoumKey::new_with_wrapped(account_id, type_script);
    Some(account_key)
}

mod cast {
    use ckb_standalone_types::packed as standalone;
    use ckb_types::packed as core_types;

    pub fn standalone_transaction_ref(s: &core_types::Transaction) -> &standalone::Transaction {
        unsafe {
            core::mem::transmute(s)
        }
    }

    // Same thing, but other direction.
    pub fn core_types_transaction_ref(s: &standalone::Transaction) -> &core_types::Transaction {
        unsafe {
            core::mem::transmute(s)
        }
    }

    pub fn standalone_transaction(s: core_types::Transaction) -> standalone::Transaction {
        unsafe {
            core::mem::transmute(s)
        }
    }

    // Same thing, but other direction.
    pub fn core_types_transaction(s: standalone::Transaction) -> core_types::Transaction {
        unsafe {
            core::mem::transmute(s)
        }
    }
}

// Wrapper which provides compatibility ckb_types instead of ckb_standalone_types
pub fn rebase(tx: Transaction, latest_states: &AccountCellMap) -> Result<Transaction, RebaseError>{
    let tx = cast::standalone_transaction(tx);
    let latest_states = latest_states.inner.clone();
    let res = rebase_auction::rebase(tx, latest_states);
    res.map(cast::core_types_transaction)
}
