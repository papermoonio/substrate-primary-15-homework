import { ApiPromise, WsProvider } from '@polkadot/api';
import Keyring from "@polkadot/keyring";
import { cryptoWaitReady, mnemonicGenerate } from "@polkadot/util-crypto";


import * as readline from 'readline';


// Define two common variables for the following functions:
// 1. api: Getting apis from network.
// 2. account: 
let api;
let accountForUsage;

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

async function connectToNetwork() {
    const wsProvider = new WsProvider('wss://rpc.polkadot.io');
    try {
        api = await ApiPromise.create({ provider: wsProvider });
        console.log('Connected to Polkadot network');
    } catch (error) {
        console.log(error);
        return;
    }
}

async function createAccount() {
    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(mnemonic, { name: 'DotWallet Account' });
    console.log("Account created: ", account.address);
    console.log("Mnemonic: ", mnemonic);
    return account;
}


async function showBalance() {
    if (!accountForUsage) {
        console.log("Please create or connect an account first.");
        return;
    }

    const { data: { free: balance } } = await api.query.system.account(accountForUsage.address);
    console.log(`Balance of ${account.address} : ${balance.toString()} units.`);
}


async function transfer() {
    if (!accountForUsage) {
        console.log("Please create or connect an account first.");
        return;
    }

    rl.question("Please enter the amount you want to transfer: ", async (amount) => {
        rl.question("Please enter the address you want to transfer to: ", async (address) => {
            const transfer = api.tx.balances.transfer(address, amount);
            const hash = await transfer.signAndSend(accountForUsage);
            console.log(`Transfer transaction hash: ${hash}`);
        });
    });
}

async function hintIncomeTransfers() {
    if (!accountForUsage) {
        console.log("Please create or connect an account first.");
        return;
    }

    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            if (api.events.balances.Transfer.is(event)) {
                const [from, to, amount] = event.data;
                if (to.toString() === accountForUsage.address) {
                    console.log(`Incoming transfer: ${amount} units from ${from}`);
                }
            }
        });
    })
}

async function callPoETransactions() {
    if (!accountForUsage) {
        console.log("Please create or connect an account first.");
        return;
    }

    rl.question('Enter the claim (string to store on-chain): ', async (claim) => {
        const claim_hash = api.createType('Hash', api.registry.hash(claim).toHex());
        const tx = api.tx.poe.createClaim(claim_hash);
        const hash = await tx.signAndSend(accountForUsage);
        console.log('PoE transaction sent with hash', hash.toHex());
    });
}

async function main() {
    await connectToNetwork();
    console.log("Welcome to use DotWallet!");
    console.log("Please type a number: ");
    console.log('1. Create Account');
    console.log('2. Show Balance');
    console.log('3. Transfer');
    console.log('4. Hint Income Transfers');
    console.log('5. Call PoE Transactions');
    console.log('6. Quit');

    rl.question('Option: ', async (input) => {
        switch (input.trim()) {
            case '1':
                await createAccount();
                break;
            case '2':
                await showBalance();
                break;
            case '3':
                await transfer();
                break;
            case '4':
                await hintIncomeTransfers();
                break;
            case '5':
                await callPoETransactions();
                break;
            case '6':
                rl.close();
                process.exit(0);
            default:
                console.log('Invalid input. Please try again.');
                break;
        }
        main();
    });

}

main().catch(console.error);