import { Command } from 'commander';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { mnemonicGenerate } from '@polkadot/util-crypto';

class PolkadotWallet {
    private localUrl = "ws://127.0.0.1:9944";
    private api: ApiPromise | null = null;
    private keyring: Keyring = new Keyring({ type: 'sr25519' });
    private currentAccount: any = null;
    private transferMethod: 'transferAllowDeath' | 'transferKeepAlive' | null = null;

    async connect() {
        console.log('Connecting to Polkadot...');
        const provider = new WsProvider(this.localUrl);
        try {
            this.api = await ApiPromise.create({ provider });
            await this.api.isReady;
            console.log('Connected.');

            const chain = await this.api.rpc.system.chain();
            console.log(`Chain: ${chain.toString()}`);

            this.transferMethod = this.api.tx.balances.transferAllowDeath
                ? 'transferAllowDeath'
                : this.api.tx.balances.transferKeepAlive
                    ? 'transferKeepAlive'
                    : null;

            if (!this.transferMethod) throw new Error('No suitable transfer method found');
        } catch (error) {
            this.handleError('Connection failed', error);
            process.exit(1);
        }
    }

    create() {
        const mnemonic = mnemonicGenerate();
        const account = this.keyring.addFromMnemonic(mnemonic);
        console.log('New account created:');
        console.log(`Address: ${account.address}`);
        console.log(`Mnemonic: ${mnemonic}`);
        console.log('Store your mnemonic securely!');
        this.currentAccount = account;
    }

    async balance(address: string) {
        if (!this.api) return this.apiNotInitialized();

        try {
            const now = await this.api.query.timestamp.now();
            const { nonce, data: { free: balance } } = await this.api.query.system.account(address) as any;
            console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);
        } catch (error) {
            this.handleError('Failed to get balance', error);
        }
    }

    async send(recipient: string, amount: string) {
        if (!this.currentAccount) return console.log('No account selected.');
        if (!this.api) return this.apiNotInitialized();
        const BOB = this.keyring.addFromUri("//Bob")

        try {
            const txHash = await this.api.tx.balances
                .transfer(recipient, amount)
                .signAndSend(this.currentAccount, (result: { status: { isInBlock: any; asInBlock: any; isFinalized: any; asFinalized: any; }; }) => {
                    console.log(`Current status is ${result.status}`);

                    if (result.status.isInBlock) {
                        console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                    } else if (result.status.isFinalized) {
                        console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                        txHash();
                    }
                });
        } catch (error) {
            this.handleError('Transfer failed', error);
        }
    }

    async watch(address: string) {
        if (!this.api) return this.apiNotInitialized();

        console.log('Watching for balance changes...');
        await this.api.query.system.account(address, (account: { nonce: number; data: { free: String; } }) => {
            console.log(`free balance is ${account.data.free}  and a nonce of ${account.nonce}`);
        });
    }



    private handleError(message: string, error: unknown) {
        if (error instanceof Error) {
            console.error(`${message}: ${error.message}`);
        } else {
            console.error(message, error);
        }
    }

    private apiNotInitialized() {
        console.error('API not initialized. Connect to the network first.');
    }
}

const program = new Command();
const wallet = new PolkadotWallet();

program
    .name('polkadot-wallet')
    .description('CLI for managing a Polkadot wallet')
    .version('1.0.0');

program
    .command('connect')
    .description('Connect to the Polkadot network')
    .action(() => wallet.connect());

program
    .command('create')
    .description('Create a new Polkadot account')
    .action(() => wallet.create());

program
    .command('balance <address>')
    .description('Get the balance of an address')
    .action((address: string) => wallet.balance(address));

program
    .command('send <recipient> <amount>')
    .description('Send funds to a recipient')
    .action((recipient: string, amount: string) => wallet.send(recipient, amount));

program
    .command('watch <address>')
    .description('Watch balance changes for an address')
    .action((address: string) => wallet.watch(address));


program.parse(process.argv);
