const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { web3Accounts, web3Enable, web3FromAddress } =require('@polkadot/extension-dapp');
const readline = require('readline');
const { mnemonicGenerate, cryptoWaitReady } = require('@polkadot/util-crypto');
let api,currentAccount;


/**
 * 创建账户
 * @returns {Promise<void>}
 */
async function createAccount() {
    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr1582' });
    const account = keyring.addFromUri(mnemonic, { name: '新户' });
    currentAccount = account;
}
/**
 * 链接账户
 * @returns {Promise<void>}
 */
async function connectToNetwork() {
    const provider = new WsProvider('wss://rpc.polkadot.io');
    api = await ApiPromise.create({ provider });
    console.log('Connected to Polkadot network');
}
/**
 * 账户信息
 * @returns {Promise<void>}
 */
async function showBalance() {
    if (!currentAccount) {
        console.log('Please create or load an account first');
        return;
    }
    const { data: { free: balance } } = await api.query.system.account(currentAccount.address);
    console.log(`Balance of ${currentAccount.address}: ${balance.toString()} units`);
}
/**
 * 交易
 * @returns
 */
async function transfer() {
    const transfer = api.tx.balances.transfer(to, amount);
    const hash = await transfer.signAndSend(from);
    console.log(`Transfer hash: ${hash.toHex()}`);
}


async function callPoETransaction() {
    if (!currentAccount) {
        return;
    }
    rl.question('Enter the claim (string to store on-chain): ', async (claim) => {
        const claimHash = api.createType('Hash', api.registry.hash(claim).toHex());
        const tx = api.tx.poe.createClaim(claimHash);
        const hash = await tx.signAndSend(currentAccount);
    });
}

