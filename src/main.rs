use revm::{
    primitives::{address, AccountInfo, TxEnv, Address, U256, Bytes, Bytecode, Output},
    InMemoryDB, EVM,
};

use ethers_contract::{BaseContract, Lazy};
use ethers_core::abi::{parse_abi,Token,ethabi};
use std::{fs::File, io::Write, path::Path, fs, include_str};
use std::str::FromStr;


fn main() -> eyre::Result<()> {
    // Read from the untrusted host via a Gramine-mapped file
    simulate()?;
    Ok(())
}

pub static ANDROMEDA_CODE: Lazy<Bytes> = Lazy::new(|| {
    include_str!("examples_Andromeda_sol_Andromeda.bin")
	.parse().unwrap()});

pub static addrA : Address = address!("4838b106fce9647bdf1e7877bf73ce8b0bad5f97");    
pub static addrB : Address = address!("F2d01Ee818509a9540d8324a5bA52329af27D19E");

fn get_code() -> eyre::Result<Bytes> {
    // This is gross, but I'm trying to remove the "constructor" from the bytecode
    // generated from solc
    let mut db = InMemoryDB::default();    
    let mut evm = EVM::new();
    let info = AccountInfo {
	code: Some(Bytecode::new_raw((*ANDROMEDA_CODE.0).into())),
        ..Default::default()
    };
    db.insert_account_info(addrB, info);
    evm.database(db);
    evm.env.tx = TxEnv {
        caller: addrA,
        transact_to: revm::primitives::TransactTo::Call(addrB),
	data: revm::primitives::Bytes::from(ANDROMEDA_CODE.0.clone()),
        ..Default::default()
    };
    let result = evm.transact_ref()?;
    //dbg!(&result);
    match result.result.output() {
	Some(o) => Ok(o.clone()),
	_ => {todo!()}
    }
}

fn simulate() -> eyre::Result<()> {
    let mut db = InMemoryDB::default();

    let code = get_code()?;
    let mut evm = EVM::new();
    let info = AccountInfo {
	code: Some(Bytecode::new_raw(code)),
        ..Default::default()
    };
    db.insert_account_info(addrB, info);
    evm.database(db);

    let abi = BaseContract::from(parse_abi(&[
        "function localRandom() public returns (bytes32)",
        "function attestSgx(bytes) public returns (bytes)",
    ])?);

    //////////////////////////
    // Suave.localRandom()
    //////////////////////////
    {
	let calldata = abi.encode("localRandom", ())?;
	evm.env.tx = TxEnv {
            caller: addrA,
            transact_to: revm::primitives::TransactTo::Call(addrB),
	    data: revm::primitives::Bytes::from(calldata.0),
            ..Default::default()
	};
	let result = evm.transact_ref()?;
	dbg!(&result.result.output());
    }

    //////////////////////////
    // Suave.attestSgx("hello")
    //////////////////////////
    {
	let calldata = abi.encode("attestSgx", (Token::Bytes("hello".as_bytes().to_vec()),))?;
	evm.env.tx = TxEnv {
            caller: addrA,
            transact_to: revm::primitives::TransactTo::Call(addrB),
	    data: revm::primitives::Bytes::from(calldata.0),
            ..Default::default()
	};
	let result = evm.transact_ref()?;
	
	// FIXME: why doesn't this work? the workaround is fine
	//let decoded_output = abi.decode_output("attestSgx", &result.result.output().unwrap())?;
	//dbg!(decoded_output);
	
	let decoded = ethabi::decode(&[ethabi::ParamType::Bytes], result.result.output().unwrap())?;
	let quote = match &decoded[0] {
	    Token::Bytes(b) => b,
	    _ => todo!()
	};
	let hex : String = quote.iter().map(|byte| format!("{:02x}", byte)).collect();
	dbg!(hex);
    }

    Ok(())
}
