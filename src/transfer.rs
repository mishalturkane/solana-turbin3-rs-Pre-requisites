use solana_client::rpc_client::RpcClient;
use solana_program::{ pubkey::Pubkey, system_instruction::transfer };
use solana_sdk::{ signature::{Signer, read_keypair_file }, transaction::Transaction };
use std::str::FromStr;
use solana_program::hash::hash;
pub fn transfer_sol() {
    let keypair = read_keypair_file("./dev_wallet.json").expect("Could not find wallet file");
    let pubkey = keypair.pubkey();

    let message_bytes = b"verify my solana keypair!";

    let sig = keypair.sign_message(message_bytes);

    let sig_hashed = hash(sig.as_ref());

    //after that we can very the signature , using the defaut implementauin

    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    }

    //define our turbin3 public key

    let to_pubkey = Pubkey::from_str("Ub3GcPyKzrZr5jxjin1wL494Gi5BRPLQ2mGyrbkyZqx").unwrap();

    //creating a devnet conection
    const RPC_URL:&str = "https://api.devnet.solana.com/";
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
