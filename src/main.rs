use clap::Parser;
use std::io;

use acme_lib::{create_p256_key,create_rsa_key};
use suave_andromeda_revm::StatefulExecutor;
use openssl::x509::{X509Req, X509ReqBuilder, X509};
use openssl::x509::extension::SubjectAlternativeName;
use openssl::pkey::{self, PKey};
use openssl::hash::MessageDigest;
use openssl::stack::Stack;
use acme_lib::Result;

use rouille::{Request,Response,Server};

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
}

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();
    let mut service = StatefulExecutor::new_with_rpc(cli_args.rpc.clone());

    let pkey = create_rsa_key(2048);
    let csr = create_csr(&pkey);
    println!("{}", String::from_utf8(pkey.rsa().unwrap().private_key_to_pem().unwrap()).unwrap());
    println!("{}", String::from_utf8(csr.unwrap().to_pem().unwrap()).unwrap());

    Server::new_ssl("0.0.0.0:5001",
		    move |request| {
/*
			service
			    .execute_command(command, cli_args.trace)
			    .match
			{
			    Ok(res) => println!("{:?}", res),
			    Err(e) => println!("{:?}", e),
			}
*/			
			Response::text("hello world")
		    },
		    include_bytes!("../ssl-cert.pem").to_vec(),
		    include_bytes!("../ssl-key.pem").to_vec(),
    )
	.expect("Failed to start server")
        .run();    
    
    /*
    Usage plan:
    1. Advance the chain
    2. If not bootstrapped, create a key and certificate request
    2a. Out of band, satisfy the certificate request
    2b. Post the certificate request on chain
    3. Support onboarding
    // If not bootstrapped:
    */
    
    // TODO: probably doesnt work due to async
    loop {
        let mut input_buf = String::new();
        io::stdin().read_line(&mut input_buf).expect("");

        // We support two commands: advance <block number|latest|empty(latest)> and execute <TxEnv json>
	let input = input_buf.trim();

        let (command, args) = match input.split_once(' ') {
            Some((command, args)) => (command, Some(args)),
            None => (input, None),
        };

        match command {
	    "bootstrap" => {
		// Create and store a TLS certificate.
	    }


	    "load" => {
		// 
	    }
	    

            "serve" => {
		// Start a new thread to serve a TLS endpoint
	    }


	    // Fallback to the "Stateful Exec" service.
	    _ => match service
		.execute_command(command, cli_args.trace)
		.await
            {
		Ok(res) => println!("{:?}", res),
		Err(e) => println!("{:?}", e),
            }
	    
	}
    }
}
