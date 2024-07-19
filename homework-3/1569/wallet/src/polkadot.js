import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate, cryptoWaitReady } from '@polkadot/util-crypto';

//**  const WS_PROVIDER = 'wss://westend-rpc.polkadot.io';  // Westend 测试网的WS地址


const WS_PROVIDER = 'wss://polkadot-rpc.publicnode.com';  // Westend 测试网的WS地址

let api;
//获取连接节点的句柄
export const getApi = async () => {
  if (!api) {
    const provider = new WsProvider(WS_PROVIDER);
    api = await ApiPromise.create({ provider });
  }
  return api;
};
// 创建账户
export const createAccount = async () => {
  await cryptoWaitReady();
  const keyring = new Keyring({ type: 'sr25519' });
  const mnemonic = mnemonicGenerate();
  const pair = keyring.addFromMnemonic(mnemonic);
  return { pair, mnemonic };
};
// 获取余额
export const getBalance = async (address) => {
  const api = await getApi();
  const { data: { free } } = await api.query.system.account(address);
  return free.toHuman();
};
// 转账
export const transfer = async (sender, recipient, amount) => {
  const api = await getApi();
  const transfer = api.tx.balances.transfer(recipient, amount);
  const hash = await transfer.signAndSend(sender);
  return hash;
};
// 订阅地址变化
export const subscribeToBalanceChanges = async (address, callback) => {
  const api = await getApi();
  api.query.system.account(address, ({ data: { free } }) => {
    callback(free.toHuman());
  });
};

// 根据区块高度，查询区块hash值
export const callPoEChainTransaction = async (hashCode) => {
  const api = await getApi();
  const txInfo = await api.rpc.chain.getBlockHash(hashCode);
  console.log(txInfo);
  return txInfo;
};
