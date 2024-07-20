import localCache from '../utils/localCache';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { createAccount, restoreAccount, getAddress, createApi, getBalance, subscribe, Account, transfer, createClaim } from '../apis';

const apis = {};

class WalletContext {
  constructor() {
    this.wallets = [];
    this.idx = 0;
    this.ready = cryptoWaitReady().then(() => this.restoreWallet());
  }
  restoreWallet() {
    const localWallets = localCache.getItem('__wallets__');
    if (localWallets) {
      localWallets.forEach((temp) => {
        this.importWallet(temp.mnemonic, true);
      });
    }
  }
  async generateWallet(nums = 12) {
    // @ts-ignore
    const account = await createAccount(nums);
    this.importWallet(account.mnemonic);
    return account;
  }
  importWallet(words, disableSave) {
    const account = restoreAccount(words);
    this.idx++;
    const name = `Account${this.idx}`;
    account.name = name;
    this.wallets.push(account);
    if (disableSave) return account;
    localCache.setItem('__wallets__', this.wallets);
    return account;
  }

  loadData() {
    return this.ready;
  }
  getAddress(account, registryNum) {
    return getAddress(account.pair, registryNum);
  }
  async getBalance(network, address) {
    const api = await this.resolveApi(network);

    return getBalance(api, address);
    // return val.div(10**decimals);
  }

  async resolveApi(network) {
    if (!apis[network]) {
      apis[network] = await createApi(network);
    }
    return apis[network];
  }

  async transfer(network, account, toaddress, amount) {
    const api = await this.resolveApi(network);
    const hash = await transfer(api, account, toaddress, amount);
    return hash;
  }

  async subscribe(network, address, listener) {
    const api = await this.resolveApi(network);
    subscribe(api, address, listener);
  }
  async callPoeSend(account, claimText) {
    const api = await this.resolveApi('local');
    const hash = await createClaim(api, account, claimText);
    return hash;
  }
}

// 1. 可以新建帐号，查看余额
// 2. 显示钱包地址
// 3. 转账到其他帐号
// 4. 入账提醒
// 5. 调用特定Substrate链的交易 （PoE链）
// 6. 参考知名钱包的任何功能 （可选）

export const walletContext = new WalletContext();
