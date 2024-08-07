import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate, cryptoWaitReady } from '@polkadot/util-crypto';
import './styles.css';

let api;
const accounts = [];

async function initApi() {
    const provider = new WsProvider('wss://rpc.polkadot.io');
    api = await ApiPromise.create({ provider });
    console.log('API 已初始化');
}

// 初始化 API 并设置事件监听器
initApi().then(() => {
    document.getElementById('createAccount').addEventListener('click', createAccount);
    updateAccountSelects();
}).catch(console.error);


document.getElementById('createAccount').addEventListener('click', createAccount);

async function createAccount() {
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.addFromUri(mnemonic);

    const account = { address: pair.address, mnemonic };
    accounts.push(account);

    const accountItem = document.createElement('div');
    accountItem.className = 'account-item';
    accountItem.textContent = `${pair.address.slice(0, 10)}...`;
    accountItem.addEventListener('click', () => showAccountDetails(account));

    document.getElementById('accountList').appendChild(accountItem);
    updateAccountSelects();
}

async function showAccountDetails(account) {
    const balance = await getBalance(account.address);
    const detailsHtml = `
        <p><strong>地址:</strong> ${account.address}</p>
        <p><strong>助记词:</strong> ${account.mnemonic}</p>
        <p><strong>余额:</strong> ${balance}</p>
        <button id="transferButton">转账</button>
    `;
    document.getElementById('accountDetails').innerHTML = detailsHtml;
    document.getElementById('transferButton').addEventListener('click', () => showTransferModal(account));
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

function showTransferModal(fromAccount) {
    const modal = document.getElementById('transferModal');
    const closeBtn = modal.querySelector('.close');
    const form = document.getElementById('transferForm');
    const fromSelect = document.getElementById('fromAccount');
    const toSelect = document.getElementById('toAccount');

    fromSelect.value = fromAccount.address;
    modal.style.display = 'block';

    closeBtn.onclick = () => modal.style.display = 'none';
    window.onclick = (event) => {
        if (event.target == modal) {
            modal.style.display = 'none';
        }
    };

    form.onsubmit = async (e) => {
        e.preventDefault();
        const toAddress = toSelect.value;
        const amount = document.getElementById('amount').value;
        await transfer(fromAccount, toAddress, amount);
        modal.style.display = 'none';
    };
}

async function transfer(fromAccount, toAddress, amount) {
    if (!api) {
        console.error('API 未初始化');
        return;
    }

    const keyring = new Keyring({ type: 'sr25519' });
    const sender = keyring.addFromUri(fromAccount.mnemonic);

    try {
        const transfer = api.tx.balances.transferKeepAlive(toAddress, amount * 1e12); // 转换为 Planck
        const hash = await transfer.signAndSend(sender);
        console.log('转账成功，交易哈希:', hash.toHex());
        alert('转账成功！');
        showAccountDetails(fromAccount); // 刷新账户详情
    } catch (error) {
        console.error('转账失败:', error);
        alert('转账失败，请查看控制台了解详情。');
    }
}

function updateAccountSelects() {
    const fromSelect = document.getElementById('fromAccount');
    const toSelect = document.getElementById('toAccount');
    fromSelect.innerHTML = '';
    toSelect.innerHTML = '';

    accounts.forEach(account => {
        const option = document.createElement('option');
        option.value = account.address;
        option.textContent = `${account.address.slice(0, 10)}...`;
        fromSelect.appendChild(option.cloneNode(true));
        toSelect.appendChild(option);
    });
}

// 初始化账户选择下拉列表
updateAccountSelects();