use clap::{Parser, Subcommand};
use polkadot_wallet_cli::wallet;

#[derive(Parser, Debug)]
#[clap(name = "wallet_cli", version = "1.0", author = "daijinwei")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 创建钱包
    CreateWallet,
    
    /// 列出钱包
    ListWallets,

    /// 列出钱包里的余额
    BalanceWallet{
        #[clap(short, long)]
        /// 账户名
        name: String,
    },
    /// 转账给其他账户
    Transfer {
        #[clap(short, long)]
        /// 发送账户名
        from: String,
        
        #[clap(short, long)]
        /// 接收账户名
        to: String,
        
        #[clap(short, long)]
        /// 转账金额
        amount: u128,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateWallet => {
            println!("创建钱包");
            wallet::account::create()?;
        }
        Commands::ListWallets => {
            println!("列出所有钱包");
            wallet::account::list()?;
        }
        Commands::BalanceWallet { name }=> {
            wallet::account::get_balance(name.as_str()).await?;
        }
        Commands::Transfer { from, to, amount } => {
            println!("从账户 {} 转账到账户 {}，金额 = {}", from, to, amount);
            wallet::account::transfer(amount).await?;
        }
    }
    Ok(())
}