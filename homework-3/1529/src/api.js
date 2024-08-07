import { ApiPromise, WsProvider } from '@polkadot/api';
import { mnemonicGenerate, cryptoWaitReady } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/keyring';

let api;
let currentAccount;

export default {
  async init() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    api = await ApiPromise.create({ provider });
    console.log('Connected to Polkadot network');
  },

  async createAccount() {
    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(mnemonic, { name: 'New Account' });
    currentAccount = account;
    return {
      address: account.address,
      mnemonic: mnemonic,
    };
  },

  async getBalance(address) {
    const { data: { free: balance } } = await api.query.system.account(address);
    return balance.toString();
  },

  async transferFunds(from, to, amount) {
    console.log(555, from, to, amount)
    const keyring = new Keyring({ type: 'sr25519' });
    const sender = keyring.addFromUri(from.mnemonic);
    const transfer = api.tx.balances.transferKeepAlive(to, amount);
    const hash = await transfer.signAndSend(sender);
    return hash.toHex();
  },

  async callPoETransaction(from, claim) {
    console.log(8888, from, claim)
    const keyring = new Keyring({ type: 'sr25519' });
    const sender = keyring.addFromUri(from.mnemonic);

    if (api.tx.poe && api.tx.poe.createClaim) {
      await api.tx.poe
        .createClaim(claim)
        .signAndSend(sender, { nonce: -1 }, result => {
          console.log(`Current status is ${result.status}`);

          if (result.status.isInBlock) {
            console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
          } else if (result.status.isFinalized) {
            console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
          }
        });
    } else {
      console.error('The "poe" module or "createClaim" transaction does not exist in the Substrate chain.');
    }
  },
  
  async depositFunds(to, amount) {
    console.log(111, to, amount)
    const keyring = new Keyring({ type: 'sr25519' });
    const alex = keyring.addFromUri('//Alice');

    const transfer = api.tx.balances.transferKeepAlive(to, amount);
    await transfer.signAndSend(alex);
  },

  listenForAccountActivity(callback) {
    api.query.system.events((events) => {
      events.forEach((record) => {
        const { event } = record;
        if (api.events.balances.Transfer.is(event)) {
          const [from, to, amount] = event.data;
          if (to.toString() === currentAccount.address) {
            callback(`Incoming transfer: ${amount} units from ${from}`);
          }
        }
      });
    });
  },
};