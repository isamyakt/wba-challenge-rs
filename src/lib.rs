pub mod programs;


#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction, message::Message};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{ pubkey::Pubkey, system_instruction::transfer, system_program };
    use std::str::FromStr;
    use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs};

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("dev-wallet.json").expect("err: didn't find wallet file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("err: couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("EtGf3KRUT2R21mAPCyZBXb7GFQy1sAeAfwBsHtCeBXP8").unwrap();

        const RPC_URL: &str = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
        

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                1_000_000
            )], 
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Checkout tx here:
            https:://explorer.solana.com/tx/{}/?cluster=devnet", 
            signature
        );
    }

    #[test]
    fn transfer_all_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("err: couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("EtGf3KRUT2R21mAPCyZBXb7GFQy1sAeAfwBsHtCeBXP8").unwrap();

        const RPC_URL: &str = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("failed to get balance");
        
        let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
        
        let message = Message::new_with_blockhash(
            &[transfer(
                &keypair.pubkey(), 
                &to_pubkey, 
                balance
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("failed to get fee calculator");

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance - fee,
            )], 
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Checkout tx here:
            https:://explorer.solana.com/tx/{}/?cluster=devnet", 
            signature
        );
    }

    #[test]
    fn enroll() {
        let signer = read_keypair_file("wba-wallet.json").expect("couldn't find wallet file");

        const RPC_URL: &str = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(RPC_URL);

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        let args = CompleteArgs {
            github: b"testaccount".to_vec()
        };

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");


        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),           
            &[&signer],
            blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed to send transcation");
        
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

    }
}