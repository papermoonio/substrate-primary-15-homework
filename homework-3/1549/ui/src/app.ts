import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { Header } from "@polkadot/types/interfaces/runtime/types";
import { mnemonicGenerate } from '@polkadot/util-crypto';
import readlineSync from 'readline-sync';

const local_url = "ws://127.0.0.1:9944";
//创建账号
const createAccount = () => {
  const mnemonic = mnemonicGenerate();
  const keyring = new Keyring({ type: 'sr25519' });
  const newAccount = keyring.addFromUri(mnemonic);

  console.log('address:', newAccount.address);
  console.log('mnemonic:', mnemonic);
}
//查询余额
const checkBalance = async (api: ApiPromise, address: string) => {
  const balance = await api.query.system.account(address);
  console.log(balance.toHuman())
}

//显示账号
const showAccountList = async (api: ApiPromise) => {
  const entries = await api.query.system.account.entries();
  entries.forEach(([key, value]) => {
    console.log(key.toHuman());
    console.log(value.toHuman());
  });
}
//交易
const transfer = async (api: ApiPromise, sender: any, amount: number) => {
  const address = readlineSync.question('AcceptAdress: ');
  const txHash = await api.tx.balances.transferAllowDeath(address, amount).signAndSend(sender);
  console.log('交易哈希:', txHash.toHex());
}
//监听
const listenForIncomingTransfers = async (api: ApiPromise, address: string) => {
  console.log(`正在监听地址 ${address} 的入账...`);
  api.query.system.events((events: any) => {
    events.forEach((record: any) => {
      // 获取事件数据
      const { event } = record;
      // 打印事件的详细信息
      console.log(`Event : ${event}`);
      console.log(`Event: ${event.section}.${event.method}`);
      checkBalance(api, address);
    });
  });
}
//poe创建存证
const createClaim = async (api: ApiPromise, claim: number, sender: any) => {
  const unsub = await api.tx.poemodule.createClaim(claim)
    .signAndSend(sender, (result:any) => {
      if (result.status.isInBlock) {
        console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
      } else if (result.status.isFinalized) {
        console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
        unsub();
      }
    });
}

const main = async () => {
  const wsProvider = new WsProvider(local_url);
  const api: ApiPromise = await ApiPromise.create({
    provider: wsProvider,
    types: {},
  });
  await api.isReady;
  console.log("wallet");
  while (true) {
    const action = readlineSync.question('(1) Create Account\n(2) Balance\n(3) showAccount\n(4) Transfer\n(5) listenForIncomingTransfers\n(6) PoE Transaction\n(7)exit\n');
    const keyring = new Keyring({ type: "sr25519" });
    const alice = keyring.addFromUri("//Alice");
    switch (action) {
      case '1':
        await createAccount();
        break;
      case '2':
        const address = readlineSync.question('adress: ');
        await checkBalance(api, address);
        break;
      case '3':
        await showAccountList(api);
        break;
      case '4':
        await transfer(api, alice, 1000);
        break;
      case '5':
        const listenAddress = readlineSync.question('listen adress:');
        await listenForIncomingTransfers(api, listenAddress);
        break;
      case '6':
        await createClaim(api,0x02,alice);
        break;
      case '7':
        process.exit(0);
    }
  }
};

main()
  .then(() => {
    console.log("successfully exited");
    process.exit(0);
  })
  .catch((err) => {
    console.log("error occur:", err);
    process.exit(1);
  });