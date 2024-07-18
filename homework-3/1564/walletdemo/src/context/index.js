import localCache from "../utils/localCache";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import {
  createAccount,
  restoreAccount,
  getAddress,
  createApi,
  getBalance,
} from "../apis";

const apis = {};

class WalletContext {
  constructor() {
    this.wallets = {};
    this.idx = 0;
    this.ready = cryptoWaitReady().then(() => this.restoreWallet());
  }
  restoreWallet() {
    const localWallets = localCache.getItem("__wallets__");
    if (localWallets) {
      Object.keys(localWallets).forEach((key) => {
        const temp = localWallets[key];
        this.importWallet(atob(temp.mnemonic));
      });
    }
  }
  async generateWallet(nums = 12) {
    // @ts-ignore
    const account = await createAccount(nums);
    this.importWallet(account.mnemonic);
    return account;
  }
  importWallet(words) {
    const account = restoreAccount(words);
    this.idx++;
    this.wallets[`Account${this.idx}`] = account;
    localCache.setItem("__wallets__", this.wallets);
    return account;
  }

  loadData() {
    return this.ready;
  }
  getAddress(account, registryNum) {
    return getAddress(account.pair, registryNum);
  }
  async getBalance(network, address) {
    await this.resolveApi(network);

    return getBalance(api, address);
    // return val.div(10**decimals);
  }

  async resolveApi() {
    if (!apis[network]) {
      apis[network] = await createApi(network);
    }
    return apis[network];
  }
}

// 1. 可以新建帐号，查看余额
// 2. 显示钱包地址
// 3. 转账到其他帐号
// 4. 入账提醒
// 5. 调用特定Substrate链的交易 （PoE链）
// 6. 参考知名钱包的任何功能 （可选）

export const walletContext = new WalletContext();
