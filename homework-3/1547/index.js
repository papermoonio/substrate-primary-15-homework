import { ApiPromise, WsProvider } from '@polkadot/api';
import { createType } from '@polkadot/types';

// Construct
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider: wsProvider });
import { mnemonicGenerate } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/keyring';
const keyring = new Keyring();


console.log((await api.rpc.system.chain()).toHuman());


const main = async () => {
    console.log("hello 1547");
    await api.isReady;
    console.log('api ready');


    let addr = await create();

    getBalances(addr);

    transfer(addr)

    setInterval(() => { balanceChanges(addr) }, 3 * 1000);

    callPoe()
}

//create 
const create = async () => {
    const mnemonic = mnemonicGenerate();
    const pair = keyring.addFromUri(mnemonic, { name: 'first pair' }, 'ed25519');

    console.log('Mnemonic:', mnemonic.toString());
    console.log(pair.meta.name, 'has address', pair.address);

    // type: ed25519, ssFormat: 42 (all defaults)
    const pairT = keyring.createFromUri(mnemonic.toString());

    // use the default as setup on init
    console.log('Substrate generic', pairT.address);
    let addr = pairT.address;
    // adjust the default ss58Format for Kusama
    keyring.setSS58Format(2);
    console.log('Kusama', pairT.address);
    // adjust the default ss58Format for Polkadot
    keyring.setSS58Format(0);
    console.log('Polkadot', pairT.address);

    return addr;
}

//Balance
const getBalances = async (address) => {
    const balance = await api.query.system.account(address);
    console.log(address, '::balance:', balance.data.free.toHuman());
}

const transfer = async (toAddress) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const transfer = api.tx.balances.transferAllowDeath(toAddress, 12345);
    const hash = await transfer.signAndSend(alice);
    console.log('Transfer sent with hash', hash.toHex());
}

const subscribeEvent = async (addr) => {

    // Subscribe to system events via storage
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events:`);

        // Loop through the Vec<EventRecord>
        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;
            if (api.events.balances.BalanceSet.is(event)) {
                console.log("api.events.balances.Transfer.is");


            } else if (api.events.balances.Transfer.is(event)) {
                console.log("api.events.balances.Transfer.is");
                const [from, to, amount] = event.data;
                if (to.toString() == addr || from.toString() == addr) {
                    console.log(`transfer amount:: ${amount}`);
                }
            }
        });
    });
}

//
const balanceChanges = async (addr) => {
    // Retrieve the initial balance. Since the call has no callback, it is simply a promise
    // that resolves to the current on-chain value
    let { data: { free: previousFree }, nonce: previousNonce } = await api.query.system.account(addr);

    // console.log(`${addr} has a balance of ${previousFree}, nonce ${previousNonce}`);
    // console.log(`You may leave this example running and start example 06 or transfer any value to ${addr}`);

    // Here we subscribe to any balance changes and update the on-screen value
    api.query.system.account(addr, ({ data: { free: currentFree }, nonce: currentNonce }) => {
        // Calculate the delta
        const change = currentFree.sub(previousFree);

        // Only display positive value changes (Since we are pulling `previous` above already,
        // the initial balance change will also be zero)
        if (!change.isZero()) {
            console.log(`New balance change of ${change}, nonce ${currentNonce}`);

            previousFree = currentFree;
            previousNonce = currentNonce;
        }
    });
}

//call poeModule
const callPoe = async () => {
    let str = '0x123456';
    const boundedVec = createType(api.registry, 'BoundedVec<u8, T::MaxClaimLength>', str);

    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const tx = api.tx.poeModule.createClaim(boundedVec);
    const hash = await tx.signAndSend(alice);
    console.log('CreateClaim sent with hash', hash.toHex());
}

main().catch((error) => {
    console.error('An error occurred:', error);
    process.exit(1);
});