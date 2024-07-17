import {
  naclDecrypt,
  naclEncrypt,
  mnemonicGenerate,
} from "@polkadot/util-crypto";
import { Keyring } from "@polkadot/keyring";
import localCache from "../utils/localCache";
import { stringToU8a, u8aToString } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { providers } from "../config/providers";
import BigNumber from "bignumber.js";

// create a keyring with some non-default values specified
const keyring = new Keyring({ type: "sr25519" });
let index = 0;

class WalletContext {
  constructor() {
    this.wallets = {};
    this.apis = {};
    this.ready = cryptoWaitReady().then(() => this.restoreWallet());
  }
  restoreWallet() {
    const localWallets = localCache.getItem("__wallets__");
    if (localWallets) {
      Object.keys(localWallets).forEach((key) => {
        const temp = localWallets[key];
        this.wallets[key] = {
          words: temp.words,
          pair: keyring.addFromMnemonic(atob(temp.words)),
        };
      });
    }
  }
  generateWallet(nums = 12) {
    // generate a mnemonic with default params (we can pass the number
    // of words required 12, 15, 18, 21 or 24, less than 12 words, while
    // valid, is not supported since it is more-easily crackable)
    // @ts-ignore
    return mnemonicGenerate(nums);
  }
  importWallet(words) {
    // // create & add the pair to the keyring with the type and some additional
    // // metadata specified
    index++;
    const name = "Account" + index;
    const pair = keyring.addFromUri(
      words,
      { name: "Account" + index },
      "ed25519"
    );
    this.wallets[name] = {
      pair,
      words: btoa(words),
    };
    localCache.setItem("__wallets__", this.wallets);

    // // the pair has been added to our keyring
    // console.log(keyring.pairs.length, 'pairs available');

    // // log the name & address (the latter encoded with the ss58Format)
    // console.log(pair.meta.name, 'has address', pair.address);
  }

  loadData() {
    return this.ready;
  }
  getAddress(account, registryNum) {
    const pair = account.pair;
    return keyring.encodeAddress(pair.publicKey, registryNum);
  }
  async getBalance(network, address,decimals) {
    if (!this.apis[network]) {
      // Construct
      const wsProvider = new WsProvider(providers[network]);
      const api = await ApiPromise.create({ provider: wsProvider });
      this.apis[network] = api;
    }

    const api = this.apis[network];

    // Retrieve the last timestamp
    const now = await api.query.timestamp.now();
    // Retrieve the account balance & nonce via the system module
    const { nonce, data: balance } = await api.query.system.account(address);
    console.log(network,address, "balance===",balance);
    console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);
      // const val = BigNumber.valueOf(balance);
      return balance.free;
    // return val.div(10**decimals);
  }
}

// 1. 可以新建帐号，查看余额
// 2. 显示钱包地址
// 3. 转账到其他帐号
// 4. 入账提醒
// 5. 调用特定Substrate链的交易 （PoE链）
// 6. 参考知名钱包的任何功能 （可选）

export const walletContext = new WalletContext();
