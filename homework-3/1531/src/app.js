import { ApiPromise, WsProvider } from "@polkadot/api";

import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady, mnemonicGenerate } from '@polkadot/util-crypto';

const local_url = "ws://127.0.0.1:9944";
const local_wsProvider = new WsProvider(local_url);
const local_api = await ApiPromise.create({
    provider: local_wsProvider,
    types: {},
});
await local_api.isReady;

const Alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const Bob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

//创建账户
async function createAccount() {
    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    const account = keyring.createFromUri(mnemonic, { name: 'first pair' }, 'ed25519');
    console.log(account.meta.name, account.address);
    return account;
}

// 查看余额的函数
async function getBalance(ADDR) {
    const { data: balance } = await local_api.query.system.account(ADDR);
    console.log(`Balance of ${ADDR}: ${balance.free}`);
}

//链接账户
async function connectToNetwork(url) {
    const wsProvider = new WsProvider(url);
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {},
    });
    await api.isReady;
    console.log('Connected to Polkadot network');
}


// 显示钱包地址
function showAddress(PHRASE) {
    // Some mnemonic phrase
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(PHRASE);
    console.log(`Address of ${PHRASE}: ${account.address}`);
}

// 转账
async function transferFunds(from, to, amount) {
    const transfer = local_api.tx.balances.transfer(to, amount);
    const hash = await transfer.signAndSend(from, (result) => {
        console.log(`Current status is ${result.status}`);

        if (result.status.isInBlock) {
            console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
        } else if (result.status.isFinalized) {
            console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
            transferFunds(from, to, amount);
        }
    });
}

// 入账提醒
async function receiptReminder(from, to, amount, addr) {
    // Get the nonce for the admin key
    const { nonce } = await local_api.query.system.account(addr);
    const transfer = local_api.tx.balances.transfer(to, amount);
    const hash = await transfer.signAndSend(from, { nonce }, ({ events = [], status }) => {
        console.log('Transaction status:', status.type);

        if (status.isInBlock) {
            console.log('Included at block hash', status.asInBlock.toHex());
            console.log('Events:');

            events.forEach(({ event: { data, method, section }, phase }) => {
                console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
            });
        } else if (status.isFinalized) {
            console.log('Finalized block hash', status.asFinalized.toHex());

            process.exit(0);
        }
    });
}




const main = async () => {
    const wsProvider = new WsProvider(local_url);
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {},
    });
    await api.isReady;


    console.log((await api.query.timestamp.now()).toHuman());

    const ADDR1 = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    const ADDR2 = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";


    const entries = await api.query.system.account.entries();
    entries.forEach(([key, value]) => {
        console.log(key.toHuman());
        console.log(value.toHuman());
    });

    console.log("Hello world");
}

main()
    .then(() => {
        console.log("successfully");
        process.exit(0);
    })
    .catch((err) => {
        console.log("err");
        process.exit(1);
    })