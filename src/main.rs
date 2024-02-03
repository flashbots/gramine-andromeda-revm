use clap::Parser;
use acme_lib::{Result,create_rsa_key};
use suave_andromeda_revm::StatefulExecutor;
use openssl::x509::{X509Req, X509ReqBuilder};
use openssl::x509::extension::SubjectAlternativeName;
use openssl::pkey::{self, PKey};
use openssl::hash::MessageDigest;
use openssl::stack::Stack;


pub fn create_csr(pkey: &PKey<pkey::Private>) -> Result<X509Req> {
    //
    // the csr builder
    let mut req_bld = X509ReqBuilder::new().expect("X509ReqBuilder");

    let mut x509_name = openssl::x509::X509NameBuilder::new().unwrap();
    x509_name.append_entry_by_text("C", "US").unwrap();
    x509_name.append_entry_by_text("ST", "IL").unwrap();
    x509_name.append_entry_by_text("O", "n/a").unwrap();
    x509_name.append_entry_by_text("CN", "*.k37713.xyz").unwrap();
    let x509_name = x509_name.build();

    req_bld.set_subject_name(&x509_name).unwrap();
    

    // set private/public key in builder
    req_bld.set_pubkey(pkey).expect("set_pubkey");

    // set all domains as alt names
    let mut stack = Stack::new().expect("Stack::new");
    let ctx = req_bld.x509v3_context(None);
    let mut an = SubjectAlternativeName::new();
    an.dns("*.k37713.xyz");

    let ext = an.build(&ctx).expect("SubjectAlternativeName::build");
    stack.push(ext).expect("Stack::push");
    req_bld.add_extensions(&stack).expect("add_extensions");

    // sign it
    req_bld
        .sign(pkey, MessageDigest::sha256())
        .expect("csr_sign");

    // the csr
    Ok(req_bld.build())
}

#[derive(Parser)]
struct Cli {
    /// The rpc endpoint to connect to
    #[arg(short, long, default_value_t = String::from("http://127.0.0.1:8545"))]
    rpc: String,
    #[arg(short, long, default_value_t = false)]
    trace: bool,
    #[arg(short, long, default_value_t = false)]
    bootstrap: bool,
}

use warp::Filter;
use std::{sync::{Arc}};
use tokio::{io::{AsyncBufReadExt},sync::Mutex};
use suave_andromeda_revm::{andromeda_precompiles, sgx_precompiles};
use futures::future::join_all;
use async_std::{task};

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();
    let mut service = StatefulExecutor::new_with_rpc(cli_args.rpc.clone());

    /* For bootstrapping, 
       we will inject the CERT and CSR into a special place associated with 
       the contract. */
    if cli_args.bootstrap {
	let pkey = create_rsa_key(2048);
	let csr = create_csr(&pkey);
	println!("{}", String::from_utf8(pkey.rsa().unwrap().private_key_to_pem().unwrap()).unwrap());
	println!("{}", String::from_utf8(csr.unwrap().to_pem().unwrap()).unwrap());
    }

    let myservice = Arc::new(Mutex::new(service));
    let myservice2 = myservice.clone();

    // Match any request and return hello world!
    let routes = warp::any().then(move || {
	let myservice = myservice2.clone();
	async move {
	    let mut service = myservice.lock().await;
	    match service
		.execute_command("advance", false)
		.await
	    {
		Ok(res) => format!("{:?}", res),
		Err(e) => format!("{:?}", e),
	    }
	}
    });

    let warpserver = task::spawn(warp::serve(routes)
        .tls()
	.cert(include_bytes!("../ssl-cert.pem").to_vec())
	.key(include_bytes!("../ssl-key.pem").to_vec())
        .run(([0, 0, 0, 0], 5001)));

    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
    let task = task::spawn(async move {
	loop {
	    let mut buffer = Vec::new();
	    let _fut = reader.read_until(b'\n', &mut buffer).await;
	    let mut service = myservice.lock().await;	
	    match service
		.execute_command(&String::from_utf8(buffer)
				 .expect("utf8 failed")
				 .strip_suffix("\n")
				 .expect("newlin failed"), cli_args.trace)
		.await
	    {
		Ok(res) => println!("{:?}", res),
		Err(e) => println!("{:?}", e),
	    }
	}
    });
			   
    join_all(vec![warpserver, task]).await;

    /*
    Usage plan:
    1. Advance the chain
    2. If not bootstrapped, create a key and certificate request
    2a. Out of band, satisfy the certificate request
    2b. Post the certificate request on chain
    3. Support onboarding
    // If not bootstrapped:
    */

    // We support two commands: advance <block number|latest|empty(latest)> and execute <TxEnv json>
}
