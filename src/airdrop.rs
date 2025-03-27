use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{read_keypair_file, Signer}
};
pub fn send_airdrop() {

    const RPC_URL:&str = "https://api.devnet.solana.com/";
    let keypair = read_keypair_file("./dev_wallet.json").expect("Could not find wallet file");

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(),1_000_000u64){
        Ok(s)=>{
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
        },
        Err(e)=>{
            println!("Oops, something wenst wrong:{}",e.to_string());
        }

    };


}
