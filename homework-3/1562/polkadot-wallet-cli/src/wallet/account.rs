// Generate an interface that we can use from the node's metadata.
// 加载配置文件

#[allow(missing_docs)]
use subxt_signer::sr25519::dev;
use subxt::{
     OnlineClient, PolkadotConfig,
}; 

use sp_core::{sr25519, Pair,crypto::Ss58Codec};
use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::to_string_pretty;
use serde_json::from_reader;
use std::collections::HashMap;
use std::io::{self, Write}; // 确保导入了 io 和 Write
use std::fs::File;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "./artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

const WALLET_FILE: &str = "wallet.json";
const WALLETS: [&str; 2] = ["daijinwei", "onblock"];
const INIT_BALANCE: f64= 100000.0;


/// 表示一个账户
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Account {
    address: String, // 账户地址
    balance: f64,    // 账户余额
}

impl Account {
    fn new(address: &str, initial_balance: f64) -> Self {
        Account {
            address: address.to_string(),
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= self.balance {
            self.balance -= amount;
            Ok(())
        } else {
            Err("余额不足".to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
struct Wallet {
    name: String,
    mnemonic: String,
    account: Account,
}

impl Wallet {
    fn new(name: &str, mnemonic: &str, address: &str, initial_balance: f64) -> Self {
        Wallet {
            name: name.to_string(),
            mnemonic: mnemonic.to_string(),
            account: Account::new(address, initial_balance),
        }
    }
    pub fn mnemonic(&mut self) -> Result<String, String> {
        // 执行某些操作成功
        Ok(self.mnemonic.clone())
    }

    fn deposit(&mut self, amount: f64) {
        self.account.deposit(amount);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        self.account.withdraw(amount)
    }

    fn get_balance(&self) -> f64 {
        self.account.balance
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ManageWallets {
    wallets: HashMap<String, Wallet>, // 存储钱包
}

impl ManageWallets {
    fn new() -> Self {
        ManageWallets {
            wallets: HashMap::new(),
        }
    }

    fn add_wallet(&mut self, wallet: Wallet) {
        self.wallets.insert(wallet.name.clone(), wallet);
    }

    /*
    fn remove_wallet(&mut self, name: &str) -> Option<Wallet> {
        self.wallets.remove(name)
    }

    fn get_wallet(&self, name: &str) ->  io::Result<Wallet> {
        let manager = ManageWallets::load_from_file(WALLET_FILE)?;
        match manager.wallets.get(name) {
            Some(wallet) => Ok(wallet.clone()), // 返回克隆的 Wallet
            None => Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到钱包: {}", name))),
        }
    }
     */

    fn get_balance(&self, name: &str) -> Option<f64> {
        self.wallets.get(name).map(|wallet| wallet.get_balance())
    }

    fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let json_data = to_string_pretty(self)?; // 序列化为漂亮的 JSON 字符串
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?; // 写入文件
        Ok(())
    }

    fn load_from_file(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let manage_wallets: ManageWallets = from_reader(file)?; // 从文件读取并反序列化
        Ok(manage_wallets)
    }
}

pub fn create()-> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ManageWallets::new();

    // 创建两个钱包
    for name in &WALLETS{
        // 生成钱包账户
        let seed = sr25519::Pair::generate_with_phrase(None);
        let mnemonic = seed.1;
        let address = seed.0.public().to_ss58check();
        //let private_key = hex::encode(seed.0.to_raw_vec().as_slice());
        let wallet = Wallet::new(name, mnemonic.as_str(), address.as_str(), INIT_BALANCE);
        manager.add_wallet(wallet);
    }
    manager.save_to_file(WALLET_FILE)?;

    Ok(())
}

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    // 从文件加载
    let manager = ManageWallets::load_from_file(WALLET_FILE)?;
    // 将 Wallet 序列化为美化的 JSON 字符串
    match to_string_pretty(&manager) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("序列化错误: {}", e),
    }
    Ok(())
}

pub async fn get_balance(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let manager = ManageWallets::load_from_file(WALLET_FILE)?;
    // 获取余额
    match manager.get_balance(name) {
        Some(balance) => println!("{} 的余额是: {}", name, balance),
        None => println!("未找到钱包: {}", name),
    }
    Ok(())
}

pub async fn transfer(
    amount: u128,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

     // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, amount);

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }
    Ok(())
}
