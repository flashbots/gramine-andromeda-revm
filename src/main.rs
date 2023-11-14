use revm::{
    primitives::{address, AccountInfo, TxEnv, Address, U256},
    InMemoryDB, EVM,
};

use std::{fs::File, io::Write, path::Path, fs};

// This payload should be generalized to include all the pre-state for each
// simulation.



#[derive(serde::Deserialize)]
struct Payload {
    sender: Address,
    amount: U256,
}

fn main() -> eyre::Result<()> {
    // Read from the untrusted host via a Gramine-mapped file
    let data: Payload = serde_json::from_reader(File::open("/var/sgx-revm-data/input")?)?;

    simulate(data)?;
    
    Ok(())
}

fn simulate(payload: Payload) -> eyre::Result<()> {
    let mut db = InMemoryDB::default();
    let receiver = payload.sender;
    let value = payload.amount;

    let balance = U256::from(111);
    // this is a random address
    let addr = address!("4838b106fce9647bdf1e7877bf73ce8b0bad5f97");
    let info = AccountInfo {
        balance,
        ..Default::default()
    };

    // Populate the DB pre-state,
    // TODO: Make this data witnessed via merkle patricia proofs.
    db.insert_account_info(addr, info);
    // For storage insertions:
    // db.insert_account_storage(address, slot, value)

    // Setup the EVM with the configured DB
    // The EVM will ONLY be able to access the witnessed state, and
    // any simulation that tries to use state outside of the provided data
    // will fail.
    let mut evm = EVM::new();
    evm.database(db);

    evm.env.tx = TxEnv {
        caller: addr,
        transact_to: revm::primitives::TransactTo::Call(receiver),
        value,
        ..Default::default()
    };

    let result = evm.transact_ref()?;

    assert_eq!(
        result.state.get(&addr).unwrap().info.balance,
        U256::from(69)
    );

    dbg!(&result);

    Ok(())
}
