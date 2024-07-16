import { mnemonicGenerate } from "@polkadot/util-crypto";
import { Keyring } from "@polkadot/keyring";

// create a keyring with some non-default values specified
const keyring = new Keyring({ type: "sr25519" });
let index = 0;

class WalletContext {
    constructor() {
        this.wallets = {};
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
        const pair = keyring.addFromUri(words, { name: "Account" + index }, "ed25519");
        this.wallets[name] = {
            pair,
            words: btoa(words),
        };
        // // the pair has been added to our keyring
        // console.log(keyring.pairs.length, 'pairs available');

        // // log the name & address (the latter encoded with the ss58Format)
        // console.log(pair.meta.name, 'has address', pair.address);
    }

    loadData(){
        return Promise.resolve();
    }
    getAddress(account, registryNum){
        const pair = account.pair;
        return keyring.encodeAddress(pair.publicKey, registryNum)
    }
}

// 1. 可以新建帐号，查看余额
// 2. 显示钱包地址
// 3. 转账到其他帐号
// 4. 入账提醒
// 5. 调用特定Substrate链的交易 （PoE链）
// 6. 参考知名钱包的任何功能 （可选）


export const walletContext = new WalletContext();