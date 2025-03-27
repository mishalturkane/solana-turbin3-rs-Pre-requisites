mod programs;
#[allow(unused_imports)]
use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs,
    UpdateArgs};

#[allow(unused_imports)]
use solana_program::system_program;
use solana_program::system_instruction::transfer;
#[allow(unused_imports)]
use solana_client::rpc_client::RpcClient;


#[allow(unused_imports)]
use solana_sdk::{ 
    pubkey::Pubkey,
    signature::{ Keypair, Signer , read_keypair_file },
    transaction::Transaction,
    message::Message,
 };
 use std::str::FromStr;
  #[allow(dead_code)]
       fn keygen() {
        let keypair = Keypair::new();
        println!("You've generated a new Solana wallet: {}", keypair.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", keypair.to_bytes());   
    }
    #[allow(dead_code)]
   
    fn airdrop(){
        const  RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("./dev_wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
        };

    }
    #[allow(dead_code)]
    fn transfer_sol() {
        let keypair = read_keypair_file("./dev_wallet.json").expect("Could not find wallet file");
        let pubkey = keypair.pubkey();
    
        // Signing the message
        let message_bytes = b"verify my solana keypair!";
        let sig = keypair.sign_message(message_bytes);
    
        // Verify with original message (NO hashing of the signature)
        match sig.verify(pubkey.as_ref(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
    
        //define our turbin3 public key
    
        let to_pubkey = Pubkey::from_str("Ub3GcPyKzrZr5jxjin1wL494Gi5BRPLQ2mGyrbkyZqx").unwrap();
    
        //creating a devnet conection
        const RPC_URL: &str = "https://api.devnet.solana.com/";
        let rpc_client = RpcClient::new(RPC_URL);
    
        //get recent blockhash
    
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
    
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000u64)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );
    
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
    
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }


    #[allow(dead_code)]
    fn transfer_all_sol() {
        let keypair = read_keypair_file("./dev_wallet.json").expect("Could not find wallet file");
    
        //define our turbin3 public key
    
        let to_pubkey = Pubkey::from_str("Ub3GcPyKzrZr5jxjin1wL494Gi5BRPLQ2mGyrbkyZqx").unwrap();
    
        //creating a devnet conection
        const RPC_URL: &str = "https://api.devnet.solana.com/";
        let rpc_client = RpcClient::new(RPC_URL);
    
    
        //get balance remaining
        let balance = rpc_client.get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
        
    
        //get recent blockhash
    
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
    
    
         //create a test transaction to calculete fees
    
         let message = Message::new_with_blockhash(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance,
            )], Some(&keypair.pubkey()), &recent_blockhash
            );
            
        let fees = rpc_client
        .get_fee_for_message(&message) .expect("Failed to get fee calculator");
    
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance-fees)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );
    
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
    
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
    
    #[test]  
   
    fn enroll(){

        //creating a devnet connection
        const RPC_URL: &str = "https://api.devnet.solana.com/";
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("./Turbin3-wallet.json").expect("Couldn't find wallet file");
    
        let prereq = Turbin3PrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );
    
        //complete Args
        let args = CompleteArgs {
            github: b"mishalturkane".to_vec(),
        };
    
        //get recent blockhash
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent  blockhash");
    
        // Now we can invoke the "complete" function let transaction =
        let transaction =   Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );
    
        let signature = rpc_client
                    .send_and_confirm_transaction(&transaction)
                    .expect("Failed to send transaction");
    
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
    


  