#[subxt::subxt(runtime_metadata_path = "artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

use std::{
    collections::VecDeque,
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{self, Path},
    sync::{Arc, Mutex},
};

use subxt::{
    tx::{self},
    utils::AccountId32,
    OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::dev;
use tokio::sync::Semaphore;
// use rpassword::read_password;
use serde::{Deserialize, Serialize};
use sp_core::{
    crypto::Ss58Codec,
    sr25519::{self},
    Pair,
};
use std::io;

#[derive(Debug)]

pub struct DotClientPool {
    clients: Arc<Mutex<VecDeque<OnlineClient<PolkadotConfig>>>>,
    semaphore: Arc<Semaphore>,
}
impl DotClientPool {
    //构造函数
    pub fn new(size: usize) -> Self {
        let clients = Arc::new(Mutex::new(VecDeque::with_capacity(size)));
        let semaphore = Arc::new(Semaphore::new(size));

        Self { clients, semaphore }
    }
    //初始化client pool
    pub async fn init(&self, url: &str, size: usize) -> Result<(), Box<dyn Error>> {
        let mut clients = self.clients.lock().unwrap();

        match OnlineClient::<PolkadotConfig>::from_insecure_url(url).await {
            Ok(client) => {
                clients.push_back(client);
                println!("Client successfully connected to {}", url);
            }
            Err(e) => {
                println!("Failed to connect to {}: {:?}", url, e);
                return Err(Box::new(e));
            }
        }

        // for _ in 0..size {
        //     match OnlineClient::<PolkadotConfig>::from_insecure_url(url).await {
        //         Ok(client) => {
        //             clients.push_back(client);
        //             println!("Client successfully connected to {}", url);
        //         }
        //         Err(e) => {
        //             println!("Failed to connect to {}: {:?}", url, e);
        //             return Err(Box::new(e));
        //         }
        //     }
        // }
        Ok(())
    }
    //获取client连接
    pub async fn get_client(&self) -> Result<OnlineClient<PolkadotConfig>, Box<dyn Error>> {
        self.semaphore.acquire().await.unwrap();
        let mut clients = self.clients.lock().unwrap();
        let client = clients.pop_front().expect("client pool is null");

        Ok(client)
    }
    //返回client连接到 pool
    pub fn return_client(&self, client: OnlineClient<PolkadotConfig>) {
        let mut clients = self.clients.lock().unwrap();
        clients.push_back(client);
        self.semaphore.add_permits(1);
    }
}

pub struct DotClient;

impl DotClient {
    //创建钱包
    pub fn generate(wallets: &mut MYWallet) {
        // println!("please enter password:");
        // let password = read_password().unwrap();  //用于会话使用

        let seed = sr25519::Pair::generate_with_phrase(None);
        let mnemonic = seed.1;
        let address = seed.0.public().to_ss58check();
        let private_key = hex::encode(seed.0.to_raw_vec().as_slice());
        println!("mnemonic:{}", &mnemonic);
        println!("address:{}", &address);
        // print!("private_key{}", &private_key);
        let wallet = Wallet::new(address, mnemonic);
        wallets.wallets.push(wallet);
    }
    //导入钱包
    pub fn import(wallets: &mut MYWallet, mnemonic: &String) {
        if wallets.wallets.iter().any(|mne| mne.mnemonic == *mnemonic) {
            println!("wallet is exists");
            return;
        }

        let seed = sr25519::Pair::from_phrase(&mnemonic, None).expect("invalid mnemonic!");
        let address = seed.0.public().to_ss58check();
        let private_key = hex::encode(seed.0.to_raw_vec().as_slice());

        println!("mnemonic:{}", &mnemonic);
        println!("address:{}", &address);
        println!("private_key:{}", &private_key);
        let wallet = Wallet::new(address, mnemonic.to_string());
        wallets.wallets.push(wallet);
    }
    //获取初始资金
    pub async fn init_amount(
        client: &OnlineClient<PolkadotConfig>,
        mywallet: &MYWallet,
        account_number: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //通过地址直接获取AccountId32
        let wallet = mywallet.get_wallet(account_number).unwrap();
        // let accountid: AccountId32 = wallet.address.parse()?;
        //通过助记词获取AccountId32
        let pair = sr25519::Pair::from_phrase(&wallet.mnemonic, None)
            .unwrap()
            .0;
        let accountid = AccountId32::from(pair.public());

        //请求初始资金地址
        // let dest:subxt::utils::MultiAddress<subxt::utils::AccountId32, ()> = dev::alice().public_key().into();
        let transfer_value = 1_000_000_000_000u128;
        let multi = subxt::utils::MultiAddress::from(accountid);
        let balance_transfer = polkadot::tx()
            .balances()
            .transfer_allow_death(multi, transfer_value);

        //给予初始资金地址
        // let to_pair = sr25519::Pair::from_phrase(&wallet.mnemonic, None).unwrap().0;
        let to_pair = sr25519::Pair::from_string("//Alice", None).expect("内置账户无效");
        let tx_signer = tx::signer::PairSigner::new(to_pair);

        //执行交易
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&balance_transfer, &tx_signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        let transfer_event = event.find_first::<polkadot::balances::events::Transfer>()?;
        if let Some(event) = transfer_event {
            println!("balance transfer success:{event:?}");
        }

        Ok(())
    }

    //交易
    pub async fn transfer(
        client: &OnlineClient<PolkadotConfig>,
        mywallet: &MYWallet,
        account_number: i32,
        to: String,
        amount: u128,
    ) -> Result<(), Box<dyn Error>> {
        //构建签名
        let wallet = mywallet.get_wallet(account_number).unwrap();
        let from_pair = sr25519::Pair::from_phrase(&wallet.mnemonic, None)
            .unwrap()
            .0;
        let tx_signer = tx::signer::PairSigner::<PolkadotConfig, sr25519::Pair>::new(from_pair);

        //构建AccountId32
        let to_account_id: AccountId32 = to.parse()?;
        let multi = subxt::utils::MultiAddress::from(to_account_id);
        let balance_transfer = polkadot::tx()
            .balances()
            .transfer_allow_death(multi, amount);

        //执行交易
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&balance_transfer, &tx_signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        let transfer_event = event.find_first::<polkadot::balances::events::Transfer>()?;
        if let Some(event) = transfer_event {
            println!("balance transfer success:{event:?}");
        }

        Ok(())
    }
    //查询余额
    pub async fn balance(
        client: &OnlineClient<PolkadotConfig>,
        wallets: &mut MYWallet,
        address: String,
    ) -> Result<(), Box<dyn Error>> {
        let accountid: AccountId32 = address.parse()?;
        let account_info = client
            .storage()
            .at_latest()
            .await?
            .fetch(&polkadot::storage().system().account(&accountid))
            .await?;

        let free_balance = account_info.unwrap().data.free;

        println!("Account balance: {:?}", free_balance);

        // //如果该地址属于自己钱包，就更新余额
        // if let Some(wallet) = wallets.wallets.iter_mut().find(|wallet| wallet.address == address ){
        //     wallet.amount = free_balance;
        // }

        Ok(())
    }
}

pub const WALLET_FILE_PATH: &str = "mywallets.json";
#[derive(Debug, Serialize, Deserialize)]
pub struct MYWallet {
    pub wallets: Vec<Wallet>,
}
impl Drop for MYWallet {

    //保存钱包文件
    fn drop(&mut self) {
        let result = self.save_to_file(WALLET_FILE_PATH);
    }
}
impl MYWallet {
    pub fn new() -> Self {
        Self {
            wallets: Vec::new(),
        }
    }
    pub fn add_wallet(&mut self, wallet: Wallet) {
        self.wallets.push(wallet);
    }
    pub fn search_wallet(&self) {
        let wallets = &self.wallets;
        for (i, value) in wallets.iter().enumerate() {
            println!("account:{}", i);
            println!("address:{}", value.address);
            println!("mnemonic:{}", value.mnemonic);
            println!("------------")
        }
    }
    pub fn get_wallet(&self, number: i32) -> Option<&Wallet> {
        let wallet = self.wallets.get(number as usize);
        wallet
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let path = Path::new(file_path);
        if !path.exists() {
            let mut file = File::create(file_path)?;
            file.write_all(json.as_bytes())?;
        } else {
            let mut file = OpenOptions::new().write(true).open(file_path)?;
            file.set_len(0);
            file.write_all(json.as_bytes())?;
        }

        Ok(())
    }

    pub fn read_to_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let mywallets: MYWallet = serde_json::from_str(&json)?;
        Ok(mywallets)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub mnemonic: String,
    // pub amount: u128,
}
impl Wallet {
    pub fn new(address: String, mnemonic: String) -> Self {
        Self { address, mnemonic }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pool_init() -> DotClientPool {
        let pool = DotClientPool::new(3);
        pool.init("http://127.0.0.1:9944", 3);
        pool
    }
    #[tokio::test]
    async fn test_init_amount() -> Result<(), Box<dyn Error>> {
        let pool = test_pool_init();
        let mut mywallets = if let Ok(wallets) = MYWallet::read_to_file(WALLET_FILE_PATH) {
            wallets
        } else {
            MYWallet::new()
        };
        let clien = pool.get_client().await?;
        DotClient::init_amount(&clien, &mywallets, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_connect() -> Result<(), Box<dyn Error>> {
        let mut mywallets = if let Ok(wallets) = MYWallet::read_to_file(WALLET_FILE_PATH) {
            wallets
        } else {
            MYWallet::new()
        };
        let client =
            OnlineClient::<PolkadotConfig>::from_insecure_url("ws://127.0.0.1:9944").await?;

        //通过地址直接获取AccountId32
        let wallet = mywallets.get_wallet(0).unwrap();
        // let accountid: AccountId32 = wallet.address.parse()?;
        //通过助记词获取AccountId32
        let pair = sr25519::Pair::from_phrase(&wallet.mnemonic, None)
            .unwrap()
            .0;
        let accountid = AccountId32::from(pair.public());

        //请求初始资金地址
        let dest: subxt::utils::MultiAddress<subxt::utils::AccountId32, ()> =
            dev::alice().public_key().into();
        let transfer_value = 1_000_000_000_000u128;
        let multi = subxt::utils::MultiAddress::from(accountid);
        let balance_transfer = polkadot::tx()
            .balances()
            .transfer_allow_death(multi, transfer_value);

        //给予初始资金地址
        // let to_pair = sr25519::Pair::from_phrase(&wallet.mnemonic, None).unwrap().0;
        let to_pair = sr25519::Pair::from_string("//Alice", None).expect("内置账户无效");
        let tx_signer = tx::signer::PairSigner::new(to_pair);

        //执行交易
        let event = client
            .tx()
            .sign_and_submit_then_watch_default(&balance_transfer, &tx_signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        let transfer_event = event.find_first::<polkadot::balances::events::Transfer>()?;
        if let Some(event) = transfer_event {
            println!("balance transfer success:{event:?}");
        }

        Ok(())
    }
}
