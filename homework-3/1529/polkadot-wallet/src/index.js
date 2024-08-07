import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate } from '@polkadot/util-crypto';

let api;

async function initApi() {
    const provider = new WsProvider('wss://rpc.polkadot.io');
    api = await ApiPromise.create({ provider });
}

initApi();

document.getElementById('createAccount').addEventListener('click', createAccount);

async function createAccount() {
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.addFromUri(mnemonic);

    const accountItem = document.createElement('div');
    accountItem.className = 'account-item';
    accountItem.textContent = `${pair.address.slice(0, 10)}...`;
    accountItem.addEventListener('click', () => showAccountDetails(pair.address, mnemonic));

    document.getElementById('accountList').appendChild(accountItem);
}

async function showAccountDetails(address, mnemonic) {
    const balance = await getBalance(address);
    const detailsHtml = `
        <p><strong>地址:</strong> ${address}</p>
        <p><strong>助记词:</strong> ${mnemonic}</p>
        <p><strong>余额:</strong> ${balance}</p>
    `;
    document.getElementById('accountDetails').innerHTML = detailsHtml;
}

async function getBalance(address) {
    if (!api) {
        return 'API 未初始化';
    }
    try {
        const { data: { free: balance } } = await api.query.system.account(address);
        return balance.toHuman();
    } catch (error) {
        console.error('获取余额失败:', error);
        return '获取余额失败';
    }
}