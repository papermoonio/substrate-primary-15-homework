const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { mnemonicGenerate } = require('@polkadot/util-crypto');
const readline = require('readline');

class PolkadotWallet {
  constructor() {
    this.api = null;
    this.keyring = new Keyring({ type: 'sr25519' });
    this.currentAccount = null;
    this.rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
  }

  async connect() {
    console.log('Connecting to Polkadot network...');
    const provider = new WsProvider('wss://rpc.polkadot.io');
    try {
      this.api = await ApiPromise.create({ provider });
      console.log('Successfully connected to Polkadot network');
      
      const chain = await this.api.rpc.system.chain();
      console.log('Chain:', chain.toString());
      
      if (typeof this.api.tx.balances.transferAllowDeath === 'function') {
        console.log('Will use balances.transferAllowDeath method for transfers');
        this.transferMethod = 'transferAllowDeath';
      } else if (typeof this.api.tx.balances.transferKeepAlive === 'function') {
        console.log('Will use balances.transferKeepAlive method for transfers');
        this.transferMethod = 'transferKeepAlive';
      } else {
        throw new Error('Unable to find a suitable transfer method');
      }
    } catch (error) {
      console.error('Connection failed:', error.message);
      process.exit(1);
    }
  }

  createAccount() {
    const mnemonic = mnemonicGenerate();
    const account = this.keyring.addFromMnemonic(mnemonic);
    console.log('\nNew account created:');
    console.log('Address:', account.address);
    console.log('Mnemonic:', mnemonic);
    console.log('\nPlease securely store your mnemonic!');
    this.currentAccount = account;
    return account;
  }

  async getBalance(address) {
    try {
      const { data: { free: balance } } = await this.api.query.system.account(address);
      console.log(`\nBalance for address ${address}: ${balance.toHuman()}`);
      return balance;
    } catch (error) {
      console.error('Failed to get balance:', error.message);
    }
  }

  async transfer(recipient, amount) {
    if (!this.currentAccount) {
      console.log('No account selected');
      return;
    }
    try {
      let transfer;
      if (this.transferMethod === 'transferAllowDeath') {
        transfer = this.api.tx.balances.transferAllowDeath(recipient, amount);
      } else if (this.transferMethod === 'transferKeepAlive') {
        transfer = this.api.tx.balances.transferKeepAlive(recipient, amount);
      } else {
        throw new Error('No valid transfer method set');
      }
  
      const hash = await transfer.signAndSend(this.currentAccount);
      console.log('\nTransfer sent, transaction hash:', hash.toHex());
    } catch (error) {
      console.error('Transfer failed:', error.message);
    }
  }

  async subscribeToBalanceChanges(address) {
    console.log('\nListening for balance changes...');
    await this.api.query.system.account(address, ({ data: { free: balance } }) => {
      console.log(`New balance for address ${address}: ${balance.toHuman()}`);
    });
  }

  async getNetworkInfo() {
    try {
      const [chain, nodeName, nodeVersion] = await Promise.all([
        this.api.rpc.system.chain(),
        this.api.rpc.system.name(),
        this.api.rpc.system.version()
      ]);
      console.log(`\nChain: ${chain}`);
      console.log(`Node name: ${nodeName}`);
      console.log(`Node version: ${nodeVersion}`);
    } catch (error) {
      console.error('Failed to get network info:', error.message);
    }
  }

  prompt(question) {
    return new Promise((resolve) => {
      this.rl.question(question, resolve);
    });
  }

  async run() {
    await this.connect();

    while (true) {
      console.log('\nPlease select an operation:');
      console.log('1. Create new account');
      console.log('2. View balance');
      console.log('3. Transfer');
      console.log('4. Listen for balance changes');
      console.log('5. Get network info');
      console.log('6. Exit');

      const choice = await this.prompt('Enter option number: ');

      switch (choice) {
        case '1':
          this.createAccount();
          break;
        case '2':
          if (this.currentAccount) {
            await this.getBalance(this.currentAccount.address);
          } else {
            console.log('Please create or import an account first');
          }
          break;
        case '3':
          if (this.currentAccount) {
            const recipient = await this.prompt('Enter recipient address: ');
            const amount = await this.prompt('Enter transfer amount: ');
            await this.transfer(recipient, amount);
          } else {
            console.log('Please create or import an account first');
          }
          break;
        case '4':
          if (this.currentAccount) {
            await this.subscribeToBalanceChanges(this.currentAccount.address);
          } else {
            console.log('Please create or import an account first');
          }
          break;
        case '5':
          await this.getNetworkInfo();
          break;
        case '6':
          console.log('Thank you for using, goodbye!');
          this.rl.close();
          process.exit(0);
        default:
          console.log('Invalid option, please try again');
      }
    }
  }
}

async function main() {
  const wallet = new PolkadotWallet();
  try {
    await wallet.run();
  } catch (error) {
    console.error('An error occurred:', error.message);
  }
}

main().catch(console.error);