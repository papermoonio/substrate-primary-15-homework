import { cryptoWaitReady, mnemonicGenerate } from "@polkadot/util-crypto";
import { Keyring, decodeAddress } from "@polkadot/keyring";
import { ApiPromise, WsProvider } from "@polkadot/api";
import * as ss58 from "@subsquid/ss58-codec";
import { providers } from "../config/providers.js";
import registryJson from "@substrate/ss58-registry";
import { ElNotification } from "element-plus";
import Big from 'big.js';

export class Account {
  constructor(pair, mnemonic) {
    this.mnemonic = mnemonic;
    this.pair = pair;
  }
}


export const keyring = new Keyring({ type: "sr25519" });

// 1. 可以新建帐号，查看余额
// 2. 显示钱包地址
// 3. 转账到其他帐号
// 4. 入账提醒
// 5. 调用特定Substrate链的交易 （PoE链）
// 6. 参考知名钱包的任何功能 （可选）

// 1.1 create
export async function createAccount(nums) {
  await cryptoWaitReady();
  // generate a mnemonic with default params (we can pass the number
  // of words required 12, 15, 18, 21 or 24, less than 12 words, while
  // valid, is not supported since it is more-easily crackable)
  // @ts-ignore
  const mnemonic = mnemonicGenerate(nums);
  const pair = keyring.addFromMnemonic(mnemonic);
  return new Account(pair, mnemonic);
}

export function restoreAccount(mnemonic) {
  const pair = keyring.addFromMnemonic(mnemonic);
  return new Account(pair, mnemonic);
}

// 1.2
export async function getBalance(api, address) {
  // Retrieve the last timestamp
  //   const now = await api.query.timestamp.now();
  // Retrieve the account balance & nonce via the system module
  const { data: balance } = await api.query.system.account(address);
//   const num1 = Big('12345678901234567890');
// const num2 = Big('9876543210987654321');
  const val = Big(balance.free);
  const decoded = ss58.decode(address);
  const item = registryJson.find((temp) => temp.prefix === decoded.prefix);
  const decl = Big(10 ** item.decimals[0]);
  console.log("address==balance===", val, val / decl);
  return val.div(decl).toString() + item.symbols[0];
}

// 2
export function getAddress(pair, registryNum = 0) {
  return keyring.encodeAddress(pair.publicKey, registryNum);
}

// 3

export function transfer(api, account, to, amount) {
  amount = amount * 10 ** 12;
  return new Promise(async (resolve, reject) => {
    try {
      const tx = await api.tx.balances.transferKeepAlive(to, amount);
      await tx.signAndSend(account.pair, ({ events = [], status,txHash }) => {
        console.log("events===", events, status.toString());
        
        ElNotification({
          title: "event",
          type: "success",
          message: status.toString(),
          position: "bottom-left",
        });
        debugger;
        if (status.isFinalized) {
          console.log(
            `从[from:${
              account.name
            }->to:${to} amount:${amount}]的转账成功。\n交易哈希：${txHash.toHex()}`
          );
          resolve(txHash.toHex());
        }
      });
    } catch (error) {
      console.log("error===", error);
      reject(error);
    }
  });
}

// 4
export function subscribe(api, address, listener) {
  api.query.system.account(address, ({ data: { free } }) => {
    listener(free.toHuman());
  });
}
// 5
export async function createClaim(api,account,claimText) {
  const claimHash = api.createType('Hash', api.registry.hash(claimText).toHex());
  console.log("createClaim:",claimHash,account.pair);
  const tx = api.tx.poe.createClaim(claimHash);
  const hash = await tx.signAndSend(account.pair);
  return hash.toHex();
}

// 6 other...

// create api
export async function createApi(network) {
  // Construct
  const wsProvider = new WsProvider(providers[network]);
  const api = await ApiPromise.create({ provider: wsProvider });
  return api;
}
