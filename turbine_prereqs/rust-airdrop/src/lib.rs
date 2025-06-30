#[cfg(test)]

mod tests {
    use bs58;
    use solana_client::rpc_client::{self, RpcClient};
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, system_program};
    use solana_sdk::{
        hash::hash,
        instruction::AccountMeta,
        instruction::Instruction,
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    //C'nvert Base58(Phantom) to wallet file format(JSON)
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as abase58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    //wallet to base58
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array( e.g.[12,34,...]:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]

    fn keygen() {
        //Create a new keypair
        let kp = Keypair::new();

        println!(
            "You've generated a new solana wallet:{}",
            kp.pubkey().to_string()
        );
        //6RJgKJG3zrcnQBoyp11mDBTfejrJDhEiKbs7p4Ace27V
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn claim_airdrop() {
        //import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // we'll estabilish a connection to solana devnet using the const we defined above
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens(2Billon lammports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed:{}", err);
            }
        }
    }
    #[test]
    fn transfer_sol() {
        //load  your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        //Generate a signature from keypair
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());

        //Verify signature using the public key
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        //Destination Adress(Turbine)
        let to_pubkey = Pubkey::from_str("3Sh1YAdet7frVbxfdy19cecwDpomWBPQ1WcCRxAz6BFc").unwrap();

        //connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);

        //fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // //Create and sign transaction
        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        //     Some(&keypair.pubkey()),
        //     &vec![&keypair],
        //     recent_blockhash,
        // );

        // //send transaction and print TX
        // let signature = rpc_client
        //     .send_and_confirm_transaction(&transaction)
        //     .expect("Failed to send transaction");
        // println!(
        //     "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
        //     signature
        // );

        // Get Current balance
        //     let balance = rpc_client
        //         .get_balance(&keypair.pubkey())
        //         .expect("Failed to get balance");

        //     //Buid a mock transsaction to calculate fee

        //     let message = Message::new_with_blockhash(
        //         &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        //         Some(&keypair.pubkey()),
        //         &recent_blockhash,
        //     );

        //     // Estimate transaction fee
        //     let fee = rpc_client
        //         .get_fee_for_message(&message)
        //         .expect("Failed to get fee calculator");

        //     //Create final transaction with balance minus fee

        //     let transaction = Transaction::new_signed_with_payer(
        //         &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        //         Some(&keypair.pubkey()),
        //         &vec![&keypair],
        //         recent_blockhash,
        //     );
        //     // This ensure that we leave zero lamports in wallet

        //     //Send transaction and verify

        //     let signature = rpc_client
        //         .send_and_confirm_transaction(&transaction)
        //         .expect("Failed to send final transaction");
        //     println!(
        //         "Success! Entire balance transferred: https://explorer.solana.com/tx/{}?cluster=devnet",
        //         signature
        //     );
    }

    #[test]
    fn submit() {
        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find the file");

        let mint = Keypair::new();

        let turbin3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        println!("{}", turbin3_prereq_program);
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        println!("{}", collection);

        let mpl_core_program =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();

        //get PDA (program derived adress)
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        // let authority = vec![99,111,108,108,101,99,116,105,111,110];
        let authority = Pubkey::from_str("5xstXUdRJKxRrqbJuo5SAfKf68y7afoYwTeH1FXbsA3k").unwrap();
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        //define the accounts metadata

        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),     //user signer
            AccountMeta::new(prereq_pda, false),         // PDA account
            AccountMeta::new(mint.pubkey(), true),       // mint keypair
            AccountMeta::new(collection, false),         // collection
            AccountMeta::new_readonly(authority, false), // authority PDA
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];

        //blockhash to buid the transaction:
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        //create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        //send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }
}
