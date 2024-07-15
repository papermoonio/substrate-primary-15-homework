use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct WalletCli {
    #[command(subcommand)]
    pub command: WalletCommand,
}
#[derive(Subcommand, Debug)]
pub enum WalletCommand {
    List,     //查看所有钱包
    Generate, //生成钱包
    Import {
        #[arg(long)] //导入钱包
        mnemonic: String,
    },
    Transfer {
        #[arg(long)]
        number: i32,
        #[arg(long)] //转账
        to: String,
        #[arg(long)]
        amount: u128,
    },
    InitAmount {
        #[arg(long)]
        number: i32,
    },
    Balance {
        #[arg(long)]
        address: String,
    },
}

// pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
//     let mut hex_string = String::new();
//     for byte in bytes {
//         hex_string.push_str(&format!("{:02x}", byte));
//     }
//     hex_string
// }
