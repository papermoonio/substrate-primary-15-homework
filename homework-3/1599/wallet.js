const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const readline = require('readline');

async function main() {
    const provider = new WsProvider('https://polkadot.js.org/apps/#/chainstate');
    const api = await ApiPromise.create({ provider });

    // 新建帐号的函数
    function createAccount(seed) {
        const keyring = new Keyring({ type: 'sr25519' });
        const account = keyring.addFromUri(seed);
        console.log(`Created account with address: ${account.address}`);
        return account;
    }

    // 显示钱包地址的函数
    function showAddress(seed) {
        const keyring = new Keyring({ type: 'sr25519' });
        const account = keyring.addFromUri(seed);
        console.log(`Address of ${seed}: ${account.address}`);
    }

    // 转账到其他帐号的函数
    async function transferFunds(from, to, amount) {
        const transfer = api.tx.balances.transfer(to, amount);
        const hash = await transfer.signAndSend(from);
        console.log(`Transfer hash: ${hash.toHex()}`);
    }

    // 查看余额的函数
    async function getBalance(address) {
        const { data: { free: balance } } = await api.query.system.account(address);
        console.log(`Balance of ${address}: ${balance}`);
    }

    // 入账提醒
    api.query.system.events((events) => {
        console.log(`Received ${events.length} events:`);

        events.forEach(({ event }) => {
            if (event.section === 'balances' && event.method === 'Transfer') {
                const [from, to, amount] = event.data;
                console.log(`Received ${amount} from ${from}`);
            }
        });
    });

    // 创建一个 readline.Interface 实例
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    // 提示用户输入种子来新建帐号
    rl.question('Please enter the seed: ', (seed) => {
        const account = createAccount(seed);

        // 提示用户输入种子来显示钱包地址
        rl.question('Please enter the seed: ', (seed) => {
            showAddress(seed);

            // 提示用户输入地址来查看余额
            rl.question('Please enter the address: ', (address) => {
                getBalance(address);

                // 提示用户输入接收者地址和转账金额
                rl.question('Please enter the recipient address and amount (separated by a space): ', (input) => {
                    const [recipient, amount] = input.split(' ');
                    transferFunds(account, recipient, amount);

                    rl.close();
                });
            });
        });
    });
}

main().catch(console.error);
