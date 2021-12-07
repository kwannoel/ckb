use ckb_app_config::{ExitCode, ImportArgs};
use ckb_async_runtime::Handle;
use ckb_chain::chain::ChainService;
use ckb_instrument::Import;
use ckb_launcher::SharedBuilder;

use ckb_avoum::AccountCellMap;
use std::sync::{Arc, RwLock};


pub fn import(args: ImportArgs, async_handle: Handle) -> Result<(), ExitCode> {
    let builder = SharedBuilder::new(
        &args.config.bin_name,
        args.config.root_dir.as_path(),
        &args.config.db,
        None,
        async_handle,
    )?;
    let (shared, mut pack) = builder.consensus(args.consensus).build()?;

    let latest_states: AccountCellMap = AccountCellMap::new();
    let latest_states_hdl: Arc<RwLock<AccountCellMap>> = Arc::new(RwLock::new(latest_states));

    let chain_service = ChainService::new(shared, pack.take_proposal_table());
    let chain_controller = chain_service.start::<&str>(Some("ImportChainService"), latest_states_hdl.clone());

    // manual drop tx_pool_builder and relay_tx_receiver
    pack.take_tx_pool_builder();
    pack.take_relay_tx_receiver();

    Import::new(chain_controller, args.source)
        .execute()
        .map_err(|err| {
            eprintln!("Import error: {:?}", err);
            ExitCode::Failure
        })
}
