const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { mnemonicGenerate, cryptoWaitReady } = require('@polkadot/util-crypto');
const { read } = require('fs');
const readlineSync = require('readline');

const readline = readlineSync.createInterface({
    input: process.stdin,
    output: process.stdout
});

let api;
let currentAccount;

async function init() {
    const provider = new WsProvider('wss://rpc.polkadot.io');
    api = await ApiPromise.create({ provider });
    console.log('Init Successfully.Connected to Polkadot network');
}

async function createAccount() {
    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(mnemonic, { name: 'New Account' });
    currentAccount = account;

    console.log('New account created:');
    await showAddress();
    console.log('Mnemonic(Please save your mnemonic):', mnemonic);
}

async function showAddress() {
    if (!currentAccount) {
        console.log('Please create or load an account first');
        return;
    }
    console.log(`address is: ${currentAccount.address}`);
}

async function showBalance() {
    if (!currentAccount) {
        console.log('Please create or load an account first');
        return;
    }
    const { data: { free: balance } } = await api.query.system.account(currentAccount.address);
    console.log(`Balance of ${currentAccount.address}: ${balance.toString()} units`);
}

async function transferFunds(from, to, amount) {
    if (!currentAccount) {
        console.log('Please create or load an account first');
        return;
    }

    const transfer = api.tx.balances.transfer(to, amount);
    const hash = await transfer.signAndSend(from);
    console.log('Transfer sent with hash', hash.toHex());
}

async function listenForIncomingTransfers() {
    if (!currentAccount) {
        console.log('Please create or load an account first');
        return;
    }

    console.log(`Listening for incoming transfers to ${currentAccount.address}`);
    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            if (api.events.balances.Transfer.is(event)) {
                const [from, to, amount] = event.data;
                if (to.toString() === currentAccount.address) {
                    console.log(`Incoming transfer: ${amount} units from ${from}`);
                }
            }
        });
    });
}

async function callPoETransaction() {
    if (!currentAccount) {
        console.log('Please create or load an account first');
        return;
    }

    readline.question('Enter the claim (string to store on-chain): ', async (claim) => {
        const claimHash = api.createType('Hash', api.registry.hash(claim).toHex());
        const tx = api.tx.poe.createClaim(claimHash);
        const hash = await tx.signAndSend(currentAccount);
        console.log('PoE transaction sent with hash', hash.toHex());
    });
}

async function main() {
    if (!api) {
        await init();
    }
    console.log('----------------------------------------------------------------');
    console.log('1. Create Account(Please create a new account firstly)');
    console.log('2. Show Wallect Balance');
    console.log('3. Show Wattect Address');
    console.log('4. Transfer');
    console.log('5. Listen for Incoming Transfers');
    console.log('6. Call PoE Transaction');
    console.log('7. Exit');

    readline.question('Select an option: ', async (option) => {
        console.log('----------------------------------------------------------------');
        switch (option) {
            case '1':
                await createAccount();
                break;
            case '2':
                await showBalance();
                break;
            case '3':
                await showAddress();
                break;
            case '4':
                const from = readline.question('Enter sender address: ');
                const to = readline.question('Enter recipient address: ');
                const amount = readline.question('Enter amount to transfer: ');
                await transferFunds(from, to, amount);
                break;
            case '5':
                await listenForIncomingTransfers();
                break;
            case '6':
                await callPoETransaction();
                break;
            case '7':
                readline.close();
                console.log('Exiting...');
                process.exit(0);
            default:
                console.log('Invalid option, please try again.');
        }
        main();
    });
}

main().catch(console.error);