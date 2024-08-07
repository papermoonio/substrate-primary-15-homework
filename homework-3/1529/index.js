// index.js
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { mnemonicGenerate, mnemonicToMiniSecret, ed25519PairFromSeed, encodeAddress } = require('@polkadot/util-crypto');

async function main() {
  const provider = new WsProvider('wss://rpc.polkadot.io');
  const api = await ApiPromise.create({ provider });

  const args = process.argv.slice(2);
  const command = args[0];

  if (command === 'create-account') {
    const mnemonic = mnemonicGenerate();
    const seed = mnemonicToMiniSecret(mnemonic);
    const keypair = ed25519PairFromSeed(seed);
    const address = encodeAddress(keypair.publicKey);

    console.log(`Mnemonic: ${mnemonic}`);
    console.log(`Address: ${address}`);
  } else if (command === 'check-balance') {
    const address = args[1];
    if (!address) {
      console.error('Please provide an address');
      return;
    }

    const { data: { free } } = await api.query.system.account(address);
    console.log(`Balance of ${address}: ${free.toHuman()}`);
  } else {
    console.log('Invalid command. Use "create-account" or "check-balance <address>".');
  }

  await api.disconnect();
}

main().catch(console.error);