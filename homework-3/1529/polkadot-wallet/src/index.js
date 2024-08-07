import './styles.css';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate, cryptoWaitReady } from '@polkadot/util-crypto';

let api;
let accounts = [];

const adjectives = ['快速的', '聪明的', '勇敢的', '友好的', '强大的', '神秘的', '欢乐的', '睿智的', '敏捷的', '温柔的'];
const nouns = ['狮子', '老虎', '熊猫', '大象', '鹰', '海豚', '狐狸', '猫头鹰', '蝴蝶', '龙'];

function generateRandomName() {
    const adjective = adjectives[Math.floor(Math.random() * adjectives.length)];
    const noun = nouns[Math.floor(Math.random() * nouns.length)];
    return `${adjective}${noun}`;
}

document.addEventListener('DOMContentLoaded', () => {
    loadAccounts();
    updateAccountSelects();
    document.getElementById('createAccount').addEventListener('click', createAccount);
    document.getElementById('networkSelect').addEventListener('change', initApi);
});

async function initApi() {
    showLoadingIndicator();
    try {
        await cryptoWaitReady();
        const networkSelect = document.getElementById('networkSelect');
        const selectedNetwork = networkSelect.value;
        const providerUrl = selectedNetwork === 'local' ? 'ws://127.0.0.1:9944' : 'wss://rpc.polkadot.io';
        const provider = new WsProvider(providerUrl);
        api = await ApiPromise.create({ provider });
        console.log('API 已初始化');
        
        console.log('可用的模块:', Object.keys(api.tx));
        if (api.tx.balances) {
            console.log('balances 模块中的可用函数:', Object.keys(api.tx.balances));
        }

        await updateAllAccountBalances();
    } catch (error) {
        console.error('API 初始化失败:', error);
    } finally {
        hideLoadingIndicator();
    }
}


async function refreshBalance(account) {
    const balanceSpan = document.getElementById('accountBalance');
    balanceSpan.textContent = '更新中...';
    
    try {
        const newBalance = await getBalance(account.address);
        account.balance = newBalance;
        balanceSpan.textContent = newBalance;
        
        // 更新账户列表中的余额显示
        updateAccountList();
        
        // 保存更新后的账户信息
        saveAccounts();
        
        console.log(`账户 ${account.name} 的余额已更新`);
    } catch (error) {
        console.error('刷新余额失败:', error);
        balanceSpan.textContent = '刷新失败';
    }
}

function showLoadingIndicator() {
    const loadingIndicator = document.createElement('div');
    loadingIndicator.id = 'loadingIndicator';
    loadingIndicator.textContent = 'API 正在初始化...';
    document.body.appendChild(loadingIndicator);
}

function hideLoadingIndicator() {
    const loadingIndicator = document.getElementById('loadingIndicator');
    if (loadingIndicator) {
        loadingIndicator.remove();
    }
}

// 更新 updateAllAccountBalances 函数
async function updateAllAccountBalances() {
    for (const account of accounts) {
        await updateAccountBalance(account);
    }
    updateAccountList();
    saveAccounts(); // 保存更新后的账户信息
    // 如果当前正在显示账户详情，也更新详情页面
    const accountDetails = document.getElementById('accountDetails');
    if (accountDetails.innerHTML !== '') {
        const displayedAddress = accountDetails.querySelector('p:nth-child(2)').textContent.split(':')[1].trim();
        const displayedAccount = accounts.find(acc => acc.address === displayedAddress);
        if (displayedAccount) {
            showAccountDetails(displayedAccount);
        }
    }
}

function loadAccounts() {
    const savedAccounts = localStorage.getItem('polkadotAccounts');
    if (savedAccounts) {
        accounts = JSON.parse(savedAccounts);
        updateAccountList();
    }
}

function saveAccounts() {
    localStorage.setItem('polkadotAccounts', JSON.stringify(accounts));
}
async function createAccount() {
    const accountName = generateRandomName();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.addFromUri(mnemonic);

    const account = { name: accountName, address: pair.address, mnemonic, balance: 'Loading...' };
    accounts.push(account);
    saveAccounts();

    addAccountToList(account);
    updateAccountSelects();

    if (api) {
        const networkSelect = document.getElementById('networkSelect');
        if (networkSelect.value === 'local') {
            await fundNewAccount(account);
        } else {
            account.balance = await getBalance(account.address);
            updateAccountList();
        }
    }
}

async function fundNewAccount(account) {
    try {
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');

        const transfer = api.tx.balances.transferKeepAlive(account.address, 10000); 
        const hash = await transfer.signAndSend(alice);
        console.log('新账户充值成功，交易哈希:', hash.toHex());
        
        // 添加延迟以确保交易被处理
        await new Promise(resolve => setTimeout(resolve, 2000));

        // 直接查询最新余额
        const { data: { free: balance } } = await api.query.system.account(account.address);
        account.balance = balance.toHuman();
        console.log('更新后的余额:', account.balance);

        updateAccountList();
        if (document.getElementById('accountDetails').innerHTML.includes(account.address)) {
            showAccountDetails(account);
        }
    } catch (error) {
        console.error('新账户充值失败:', error);
        account.balance = '充值失败';
    }
}

function addAccountToList(account) {
    const accountItem = document.createElement('div');
    accountItem.className = 'account-item';
    
    const accountInfo = document.createElement('span');
    accountInfo.textContent = `${account.name} (${account.address.slice(0, 10)}...) ${account.balance || ''}`;
    accountInfo.addEventListener('click', () => showAccountDetails(account));
    
    const deleteButton = document.createElement('button');
    deleteButton.textContent = '删除';
    deleteButton.className = 'delete-button';
    deleteButton.addEventListener('click', (e) => {
        e.stopPropagation();
        deleteAccount(account);
    });
    
    accountItem.appendChild(accountInfo);
    accountItem.appendChild(deleteButton);

    document.getElementById('accountList').appendChild(accountItem);
}

function deleteAccount(accountToDelete) {
    if (confirm('确定要删除这个账户吗？此操作不可逆。')) {
        accounts = accounts.filter(account => account.address !== accountToDelete.address);
        saveAccounts();
        updateAccountList();
        updateAccountSelects();
        document.getElementById('accountDetails').innerHTML = '';
    }
}

function updateAccountList() {
    const accountList = document.getElementById('accountList');
    accountList.innerHTML = '';
    accounts.forEach(account => addAccountToList(account));
}

async function showAccountDetails(account) {
    const balance = await getBalance(account.address);
    const detailsHtml = `
        <h2>${account.name}</h2>
        <p><strong>地址:</strong> ${account.address}</p>
        <p><strong>助记词:</strong> ${account.mnemonic}</p>
        <p><strong>余额:</strong> <span id="accountBalance">${balance}</span></p>
        <button id="refreshBalanceButton">刷新余额</button>
        <button id="transferButton">转账</button>
        <button id="fundButton">充值 (仅本地节点)</button>
    `;
    document.getElementById('accountDetails').innerHTML = detailsHtml;
    document.getElementById('transferButton').addEventListener('click', () => showTransferModal(account));
    document.getElementById('fundButton').addEventListener('click', () => fundAccount(account));
    document.getElementById('refreshBalanceButton').addEventListener('click', () => refreshBalance(account));
}

// 更新 getBalance 函数
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
async function fundAccount(account) {
    if (!api) {
        console.error('API 未初始化');
        return;
    }

    const networkSelect = document.getElementById('networkSelect');
    if (networkSelect.value !== 'local') {
        alert('充值功能仅在本地节点上可用');
        return;
    }

    try {
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');

        const transfer = api.tx.balances.transferKeepAlive(account.address, 10000000000000); // 转账 10 DOT
        const hash = await transfer.signAndSend(alice);
        console.log('充值成功，交易哈希:', hash.toHex());
        
        // 添加延迟以确保交易被处理
        await new Promise(resolve => setTimeout(resolve, 2000));

        // 直接查询最新余额
        const { data: { free: balance } } = await api.query.system.account(account.address);
        account.balance = balance.toHuman();
        console.log('更新后的余额:', account.balance);

        updateAccountList();
        if (document.getElementById('accountDetails').innerHTML.includes(account.address)) {
            showAccountDetails(account);
        }
        
        alert('充值成功！');
    } catch (error) {
        console.error('充值失败:', error);
        alert('充值失败，请查看控制台了解详情。');
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
        // 获取当前账户余额
        const { data: { free: balance } } = await api.query.system.account(fromAccount.address);
        const currentBalance = BigInt(balance);
        const transferAmount = BigInt(amount * 1e12);

        // 估算交易费用
        const info = await api.tx.balances.transferKeepAlive(toAddress, transferAmount).paymentInfo(sender);
        const fee = BigInt(info.partialFee);

        // 检查余额是否足够
        if (currentBalance < transferAmount + fee) {
            console.log(currentBalance)
            console.log(transferAmount , fee)
            throw new Error('余额不足以支付转账金额和交易费用');
        }

        const transfer = api.tx.balances.transferKeepAlive(toAddress, transferAmount);
        const hash = await transfer.signAndSend(sender);
        console.log('转账成功，交易哈希:', hash.toHex());
        
        // 添加延迟以确保交易被处理
        await new Promise(resolve => setTimeout(resolve, 2000));

        // 更新发送方和接收方的余额
        await updateAccountBalance(fromAccount);
        const toAccount = accounts.find(acc => acc.address === toAddress);
        if (toAccount) {
            await updateAccountBalance(toAccount);
        }

        alert('转账成功！');
        updateAccountList();
        if (document.getElementById('accountDetails').innerHTML.includes(fromAccount.address)) {
            showAccountDetails(fromAccount);
        }
    } catch (error) {
        console.error('转账失败:', error);
        alert('转账失败: ' + error.message);
    }
}

async function updateAccountBalance(account) {
    try {
        const { data: { free: balance } } = await api.query.system.account(account.address);
        account.balance = balance.toHuman();
        console.log(`账户 ${account.name} 的余额已更新为: ${account.balance}`);
    } catch (error) {
        console.error(`更新账户 ${account.name} 的余额失败:`, error);
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
        option.textContent = `${account.name} (${account.address.slice(0, 10)}...)`;
        fromSelect.appendChild(option.cloneNode(true));
        toSelect.appendChild(option);
    });
}

initApi().catch(console.error);