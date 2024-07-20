const { ApiPromise, WsProvider } = require("@polkadot/api");
const { Keyring } = require("@polkadot/keyring");
const { cryptoWaitReady, mnemonicGenerate } = require("@polkadot/util-crypto");

const LOCAL_URL = "ws://127.0.0.1:9944"; // 本地
// const LOCAL_WS_PROVIDER = new WsProvider(LOCAL_URL);
// const LOCAL_API = await ApiPromise.create({
//   provider: LOCAL_WS_PROVIDER,
//   types: {},
// });

// await LOCAL_API.isReady;

//创建钱包账户
async function create() {
  await cryptoWaitReady();
  const mnemonic = mnemonicGenerate();
  const keyring = new Keyring({ type: "sr25519", ss58Format: 2 });
  const account = keyring.createFromUri(
    mnemonic,
    { name: "first pair" },
    "ed25519"
  );
  console.log("账户名称：", account.meta.name);
  console.log("账户地址：", account.address);
  return account;
}

// 获取地址余额
async function getBalance(address) {
  const { data: balance } = await LOCAL_API.query.system.account(address);
  console.log(`地址：${address}，的余额是: ${balance.free}。`);
}

//链接账户
async function connect(url) {
  const WS_PROVIDER = new WsProvider(url);
  const API = await ApiPromise.create({
    provider: WS_PROVIDER,
    types: {},
  });
  console.log("连接 polkadot 网络成功！!");
  return API;
}

// 根据助记词获取钱包地址
function getAddressByMnemonicWord(mnemonicWord) {
  const keyring = new Keyring({ type: "sr25519" });
  const account = keyring.addFromUri(mnemonicWord);
  console.log(`助记词是：${mnemonicWord}`);
  console.log(`助记词对应的钱包地址是：${account.address}`);
}

// 转账
async function transferBalance(from, to, amount) {
  const LOCAL_API = await connect(LOCAL_URL);
  await LOCAL_API.isReady;
  const transfer = LOCAL_API.tx.balances.transfer(to, amount);
  const hash = await transfer.signAndSend(from, (result) => {
    console.log("当前状态：", result.status);
    if (result.status.isInBlock) {
      console.log("交易中的blockHash：", result.status.asInBlock);
    } else if (result.status.isFinalized) {
      console.log("交易完成的blockHash：", result.status.asFinalized);
      transferBalance(from, to, amount);
    }
  });
}

// 接收转账
async function receiveTransfers(from, to, amount, addr) {
  const LOCAL_API = await connect(LOCAL_URL);
  await LOCAL_API.isReady;
  const { nonce } = await LOCAL_API.query.system.account(addr);
  const transfer = LOCAL_API.tx.balances.transfer(to, amount);
  const hash = await transfer.signAndSend(
    from,
    { nonce },
    ({ events = [], status }) => {
      console.log("交易状态：", status.type);
      if (status.isInBlock) {
        console.log("交易中的blockHash：", status.asInBlock.toHex());
        console.log("所有的事件：");
        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log(
            "\t",
            phase.toString(),
            `: ${section}.${method}`,
            data.toString()
          );
        });
      } else if (status.isFinalized) {
        console.log("最终的区块hash：", status.asFinalized.toHex());
        // 正常退出程序
        process.exit(0);
      }
    }
  );
}

// 入口启动函数
const main = async () => {
  const API = await connect(LOCAL_URL);
  await API.isReady;

  console.log("获取当前时间戳：", (await API.query.timestamp.now()).toHuman());
  const entries = await API.query.system.account.entries();
  // 打印所有的账户信息
  entries.forEach(([key, value]) => {
    console.log(key.toHuman());
    console.log(value.toHuman());
  });
};

// 原神 启动
main()
  .then(() => {
    console.log("钱包运行成功");
    // 正常退出程序
    process.exit(0);
  })
  .catch((err) => {
    console.error();
    "钱包运行失败，原因是: " + err;
    // 异常退出程序
    process.exit(1);
  });
