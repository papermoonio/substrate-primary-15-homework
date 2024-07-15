use std::error::Error;

use clap::Parser;

mod dotclientpool;
mod wallet;
use dotclientpool::{DotClient, DotClientPool, MYWallet, WALLET_FILE_PATH};

use wallet::{WalletCli, WalletCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mywallets = if let Ok(wallets) = MYWallet::read_to_file(WALLET_FILE_PATH) {
        wallets
    } else {
        MYWallet::new()
    };

    let isempty = mywallets.wallets.is_empty();
    let pool = DotClientPool::new(3);
    pool.init("ws://127.0.0.1:9944", 3).await?;
    let clien = pool.get_client().await?;

    let args = WalletCli::parse();
    match &args.command {
        WalletCommand::List => {
            if isempty {
                println!("wallet not account");
                return Ok(());
            }
            mywallets.search_wallet();
        }
        WalletCommand::Generate => {
            DotClient::generate(&mut mywallets);
        }
        WalletCommand::Import { mnemonic } => {
            DotClient::import(&mut mywallets, mnemonic);
        }
        WalletCommand::Transfer { number, to, amount } => {
            if isempty {
                println!("wallet not account");
                return Ok(());
            }
            DotClient::transfer(&clien, &mut mywallets, *number, to.to_string(), *amount).await?;
        }
        WalletCommand::InitAmount { number } => {
            if isempty {
                println!("wallet not account");
                return Ok(());
            }
            DotClient::init_amount(&clien, &mut mywallets, *number).await?;
        }
        WalletCommand::Balance { address } => {
            DotClient::balance(&clien, &mut mywallets, address.to_string()).await?;
        }
    }

    Ok(())
}
