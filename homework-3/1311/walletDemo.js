const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { mnemonicGenerate, cryptoWaitReady } = require('@polkadot/util-crypto');
const readline = require('readline');

const r = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

let api;
let account;

async function connect() {
    const provider = new WsProvider('wss://rpc.polkadot.io');
    api = await ApiPromise.create({ provider });
    console.log('Connected to Polkadot');
}
async function main() {
    await connect();

    console.log('Welcome to Polkadot WalletDemo！');
    console.log('1. Create new account');
    console.log('2. Display balance');
    console.log('3. Display address');
    console.log('4. Transfer accounts');
    console.log('5. Transfer reminder');
    console.log('6. Call Poe Chain');
    console.log('7. Exit');

    r.question('Option: ', async (option) => {
        switch (option) {
            case '1':
                await createNewAccount();
                break;
            case '2':
                await displayBalance();
                break;
            case '3':
                await displayAddress();
                break;
            case '4':
                await transferAccounts();
                break;
            case '5':
                await transferReminder();
                break;
            case '6':
                await callPoeChain();
                break;
            case '7':
                r.close();
                process.exit(0);
            default:
                console.log('error!');
        }
        main();
    });
}

async function createNewAccount() {
    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const newAccount = keyring.addFromUri(mnemonic, { name: 'New Account' });

    console.log('New account created:');
    console.log('1 address:', newAccount.address);
    console.log('2 mnemonic:', mnemonic);
    account = newAccount;
}

async function displayBalance() {
    await checkAccount();
    const { data: { free: balance } } = await api.query.system.account(account.address);
    console.log(`Balance of ${account.address}: ${balance.toString()} units`);
}

async function displayAddress() {
    await checkAccount();
    console.log(`address is: ${account.address}`);
}

async function transferAccounts() {
    await checkAccount();

    r.question('Enter the receiving account: ', async (recipient) => {
        r.question('Enter the transfer amount: ', async (amount) => {
            const transfer = api.tx.balances.transfer(recipient, amount);
            const hash = await transfer.signAndSend(account);
            console.log('Transfer sent with hash', hash.toHex());
        });
    });
}

async function transferReminder() {
    await checkAccount();

    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            if (api.events.balances.Transfer.is(event)) {
                const [from, to, amount] = event.data;
                if (to.toString() === account.address) {
                    console.log(`Incoming transfer: ${amount} units from ${from}`);
                }
            }
        });
    });
}

async function callPoeChain() {
    await checkAccount();

    readline.question('Enter the claim (string to store on-chain): ', async (claim) => {
        const claimHash = api.createType('Hash', api.registry.hash(claim).toHex());
        const tx = api.tx.poe.createClaim(claimHash);
        const hash = await tx.signAndSend(account);
        console.log('PoE transaction sent with hash', hash.toHex());
    });
}

async function checkAccount() {
    if (!account) {
        console.log('account error!');
        return;
    }
}



main().catch(console.error);