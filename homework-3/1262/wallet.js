const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { mnemonicGenerate, cryptoWaitReady } = require('@polkadot/util-crypto');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

let api;
let currentAccount;

async function connectToNetwork() {
  const provider = new WsProvider('wss://rpc.polkadot.io');
  api = await ApiPromise.create({ provider });
  console.log('Connected to Polkadot network');
}

async function createAccount() {
  await cryptoWaitReady();
  const mnemonic = mnemonicGenerate();
  const keyring = new Keyring({ type: 'sr25519' });
  const account = keyring.addFromUri(mnemonic, { name: 'New Account' });
  console.log('New account created:');
  console.log('Address:', account.address);
  console.log('Mnemonic:', mnemonic);
  currentAccount = account;
}

async function showBalance() {
  if (!currentAccount) {
    console.log('Please create or load an account first');
    return;
  }
  const { data: { free: balance } } = await api.query.system.account(currentAccount.address);
  console.log(`Balance of ${currentAccount.address}: ${balance.toString()} units`);
}

async function transfer() {
  if (!currentAccount) {
    console.log('Please create or load an account first');
    return;
  }
  
  rl.question('Enter recipient address: ', async (recipient) => {
    rl.question('Enter amount to transfer: ', async (amount) => {
      const transfer = api.tx.balances.transfer(recipient, amount);
      const hash = await transfer.signAndSend(currentAccount);
      console.log('Transfer sent with hash', hash.toHex());
    });
  });
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
  
  rl.question('Enter the claim (string to store on-chain): ', async (claim) => {
    const claimHash = api.createType('Hash', api.registry.hash(claim).toHex());
    const tx = api.tx.poe.createClaim(claimHash);
    const hash = await tx.signAndSend(currentAccount);
    console.log('PoE transaction sent with hash', hash.toHex());
  });
}

async function main() {
  await connectToNetwork();
  
  console.log('Polkadot Wallet CLI');
  console.log('1. Create Account');
  console.log('2. Show Balance');
  console.log('3. Transfer');
  console.log('4. Listen for Incoming Transfers');
  console.log('5. Call PoE Transaction');
  console.log('6. Exit');
  
  rl.question('Select an option: ', async (option) => {
    switch (option) {
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
        await listenForIncomingTransfers();
        break;
      case '5':
        await callPoETransaction();
        break;
      case '6':
        rl.close();
        process.exit(0);
      default:
        console.log('Invalid option');
    }
    main();
  });
}

main().catch(console.error);