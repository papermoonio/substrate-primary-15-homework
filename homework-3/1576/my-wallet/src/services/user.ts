import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { hexToU8a } from '@polkadot/util';
import { strToHex } from '../utils/tool';
// Construct
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider: wsProvider });

async function newAccount(mne: any) {
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(mne);
    return account;
}

async function queryBalances(address: string) {
    const balances = await api.query.system.account(address);
    return balances.data.free.toHuman();
}

async function execTransfer(target: string, num: number) {
    console.log(target);
    const keyring = new Keyring({ type: 'sr25519' });
    const origin = keyring.addFromUri('//Alice');
    const transfer = api.tx.balances.transferAllowDeath(target, num);
    const hash = await transfer.signAndSend(origin);
    api.query.system.events((events) => {
        events.forEach((record) => {
          const { event } = record;
          if (api.events.balances.Transfer.is(event)) {
            const [from, to, amount] = event.data;
            if (to.toString() === target) {
              console.log(`转账提醒: 从 ${from} 收到 ${amount} units `);
            }
          }
        });
      });
    return hash;
}

async function addClaim(data: string) {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    // 将输入数据转换为字节数组
    let inputBytes = hexToU8a(strToHex(data));
    // 检查字节数组的长度是否为32字节（256位）
    if (inputBytes.length !== 32) {
        // 如果不是32字节，填充或截断字节数组
        const filledBytes = new Uint8Array(32);
        filledBytes.set(inputBytes);
        inputBytes = filledBytes;
    }
    const tx = api.tx.templateModule.createClaim(inputBytes);
    const hash = await tx.signAndSend(alice);
    console.log(hash);
    return hash.toHex()
}

export { newAccount, queryBalances, execTransfer, addClaim }