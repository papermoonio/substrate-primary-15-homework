import { ApiPromise, WsProvider } from '@polkadot/api';
import Keyring from "@polkadot/keyring";
import { cryptoWaitReady, mnemonicGenerate } from "@polkadot/util-crypto";


// 导入readline模块
const readline = require('readline');

// 创建readline接口实例
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

// 定义询问节点地址的函数
function askNodeAddress() {
    rl.question("1. 请输入节点地址：", (nodeAddress) => {
      console.log(`节点地址设置为：${nodeAddress}`);
        // Construct
        const wsProvider = new WsProvider(nodeAddress);
        const api = ApiPromise.create({ provider: wsProvider });
        console.log("链接成功...");
        console.log("genesisHash:");
        console.log(api.genesisHash.toHex());
      createNewWallet();
    });
  }
  
  // 定义创建新钱包的函数
  function createNewWallet() {
    console.log("2. 创建新钱包...");
    rl.question("请给钱包去给名：", (accountName) => {
        console.log(`钱包名称为：${accountName}`);
        // 创建钱包
        const mnemonic = mnemonicGenerate();
        const keyring = new Keyring({ type: 'sr25519' });
        const account = keyring.addFromUri(mnemonic, { name: accountName });
        console.log(accountName," created: ", account.address);
        console.log("Mnemonic: ", mnemonic);
        
        // 获取余额
        const { data: { free: balance } } = api.query.system.account(account.address);
        console.log(`Balance of ${accountName}: ${balance.toString()} units.`);

        askReceiveAddress();
      });
  }
  

  // 定义询问接收地址的函数
  function askReceiveAddress() {
    rl.question("3. 请输入接收地址：", (receiveAddress) => {
        rl.question("请输入转账金额：", (transferAmount) => {
            // 转账
            const txHash = api.tx.balances
            .transfer(receiveAddress, transferAmount)
            .signAndSend(account);
          console.log(`转账金额设置为：${transferAmount}`);
          console.log("txHash:",txHash);

        });
    });
  }
  
  // 开始询问节点地址
  askNodeAddress();