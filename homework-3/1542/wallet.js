// 导入必要的库
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { stringToU8a, u8aToHex } = require('@polkadot/util');
const { mnemonicGenerate } = require('@polkadot/util-crypto');
const readlineSync = require('readline-sync');

// 定义连接到的Substrate节点WebSocket地址
const WS_PROVIDER = 'wss://rpc.polkadot.io';

// 创建钱包类
class Wallet {
  constructor() {
    this.api = null;
    this.keyring = new Keyring({ type: 'sr25519' });
    this.accounts = new Map();
  }

  // 初始化API连接
  async init() {
    const provider = new WsProvider(WS_PROVIDER);
    this.api = await ApiPromise.create({ provider });
    console.log('Connected to Substrate node');
  }

  // 创建新账户
  createAccount() {
    const mnemonic = mnemonicGenerate();
    const account = this.keyring.addFromMnemonic(mnemonic);
    this.accounts.set(account.address, account);
    console.log(`New account created with address: ${account.address}`);
    console.log(`Please save your mnemonic phrase: ${mnemonic}`);
    return account;
  }

  // 显示账户地址
  showAddresses() {
    console.log('Your wallet addresses:');
    this.accounts.forEach((account, address) => {
      console.log(address);
    });
  }

  // 查询账户余额
  async getBalance(address) {
    const { data: { free: balance } } = await this.api.query.system.account(address);
    console.log(`Balance of ${address}: ${balance.toHuman()}`);
  }

  // 转账功能
  async transfer(from, to, amount) {
    const account = this.accounts.get(from);
    if (!account) {
      console.log('Sender account not found in wallet');
      return;
    }

    const transfer = this.api.tx.balances.transfer(to, amount);
    const hash = await transfer.signAndSend(account);
    console.log(`Transfer sent with hash ${hash.toHex()}`);
  }

  // 监听入账事件
  async subscribeToBalanceChanges(address) {
    await this.api.query.system.account(address, ({ data: { free: balance } }) => {
      console.log(`Balance changed for ${address}: New balance ${balance.toHuman()}`);
    });
  }

  // 调用PoE链的特定交易 (这里以创建证明为例)
  async createProof(account, claim) {
    const claimHash = u8aToHex(stringToU8a(claim));
    const tx = this.api.tx.poe.createClaim(claimHash);
    const hash = await tx.signAndSend(account);
    console.log(`Proof created with hash ${hash.toHex()}`);
  }
}

// 主函数
async function main() {
  const wallet = new Wallet();
  await wallet.init();

  while (true) {
    console.log('\n1. Create new account');
    console.log('2. Show addresses');
    console.log('3. Check balance');
    console.log('4. Transfer funds');
    console.log('5. Subscribe to balance changes');
    console.log('6. Create PoE proof');
    console.log('7. Exit');

    const choice = readlineSync.question('Choose an option: ');

    switch (choice) {
      case '1':
        wallet.createAccount();
        break;
      case '2':
        wallet.showAddresses();
        break;
      case '3':
        const addressToCheck = readlineSync.question('Enter address to check balance: ');
        await wallet.getBalance(addressToCheck);
        break;
      case '4':
        const from = readlineSync.question('Enter sender address: ');
        const to = readlineSync.question('Enter recipient address: ');
        const amount = readlineSync.question('Enter amount to transfer: ');
        await wallet.transfer(from, to, amount);
        break;
      case '5':
        const addressToWatch = readlineSync.question('Enter address to watch for balance changes: ');
        await wallet.subscribeToBalanceChanges(addressToWatch);
        break;
      case '6':
        const proofAccount = readlineSync.question('Enter account address to create proof: ');
        const claim = readlineSync.question('Enter the claim: ');
        await wallet.createProof(wallet.accounts.get(proofAccount), claim);
        break;
      case '7':
        console.log('Exiting...');
        process.exit(0);
      default:
        console.log('Invalid option, please try again.');
    }
  }
}

// 运行主函数
main().catch(console.error);

