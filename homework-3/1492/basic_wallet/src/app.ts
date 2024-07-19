import { createInterface } from "readline";
import {createAccount, generateMnemonic, readAccount, accountList, transferAndSubscribe, } from "./services";
import {connect, createKeyring, readRuntimeVersion} from "./utils/utils";
import Keyring from "@polkadot/keyring";
import { ApiPromise } from "@polkadot/api";
import { Claim_Store, Key_Store } from "./constants/constants";
import { claimList, createClaim, revokeClaim, transferClaimAndSubscribe } from "./services/claim";

const rl = createInterface({
  input: process.stdin,
  output: process.stdout,
});

const main = async () => {
  console.log("欢迎使用Omglion Wallet\n");

  // 获取连接
  const api = await connect();
  // 创建keyring
  const keyring = createKeyring();

  await readRuntimeVersion(api);
  console.log();

  while (true) {
    console.log("请根据以下列出的选项，选择您要进行的操作，输入序号数字即可：");
    console.log("1. 创建钱包账户");
    console.log("2. 查看账户");
    console.log("3. 转账");
    console.log("4. 创建存证");
    console.log("5. 撤销存证");
    console.log("6. 转移存证");
    console.log("0. 退出");
    console.log();

    // 获取用户输入
    const input = await new Promise<string>((resolve) => {
      rl.question("(输入0退出)>: ", resolve);
    });

    if (input === "0") {
      console.log("Bye!\n");
      process.exit(0);
    }

    switch (input) {
      case "1":
        await newAccount(keyring);
        break;
      case "2":
        await checkAccount(api, keyring);
        break;
      case "3":
        await transferAmount(api, keyring);
        break;
      case "4":
        await newClaim(api, keyring);
        break;
      case "5":
        await removeClaim(api, keyring);
        break;
      case "6":
        await transferClaim(api, keyring);
        break;
      default:
        console.log("无效的选项，请重新输入。");
        break;
    }
    console.log();
  }
};

main()
  .then(() => {
    console.log("Bye!");
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

// 创建账号
const newAccount = async (keyring: Keyring) => {
  const mnemonic = generateMnemonic();
  console.log("您的账号助记词是（请务必保存到安全位置）：");
  console.log(mnemonic);
  console.log("请输入您的账号名称：");
  const username = await new Promise<string>((resolve) => {
    rl.question(">: ", resolve);
  });
  console.log("正在生成您的账号...");
  const account = createAccount(keyring, mnemonic, username);
  console.log("账号创建成功，您的账号地址是：", account.address);
  // console.log(account.meta.name);

  console.log("请保存您的账号地址，以便于以后使用。");
};

// 查看账号
const checkAccount = async (api: ApiPromise, keyring: Keyring) => {
  while (true) {
    console.log("请选择要查看的账号（输入数字即可）：");
    accountList();

    const input = await new Promise<string>((resolve) => {
      rl.question("(输入0退出)>: ", resolve);
    });
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。");
      break;
    }
    // 将 input 转换为数字
    const index = parseInt(input, 10);

    if (isNaN(index) || index < 0 || index > Key_Store.length) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }

    const account = await readAccount(api, index);
    console.log("\n您的账户信息：");
    console.log(
      `户名：${account.name}\n地址：${account.address}\n余额：${account.freeBalance}`
    );
    console.log();
  }
};
async function transferAmount(api: ApiPromise, keyring: Keyring) {
  while (true) {
    console.log("已有账户列表：");
    accountList();
    let input = await new Promise<string>((resolve) => {
      rl.question("请输入待转出【账户序号】(输入0退出)>: ", resolve);
    });
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。");
      break;
    }
    // 将 input 转换为数字
    const from = parseInt(input, 10);
    if (isNaN(from) || from < 0 || from > Key_Store.length) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }

    input = await new Promise<string>((resolve) => {
      rl.question("请输入待转入【账户序号】(输入0退出)>: ", resolve);
    });
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。");
      break;
    }
    const to = parseInt(input, 10);
    if (isNaN(to) || to < 0 || to > Key_Store.length) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }

    input = await new Promise<string>((resolve) => {
      rl.question("请输入【转账金额】(输入0退出)>: ", resolve);
    });
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。");
      break;
    }

    const amount = parseInt(input, 10);
    if (isNaN(amount) || amount < 0) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }
    console.log("正在转账：");
    await transferAndSubscribe(api, keyring, from, to, amount);
    console.log();
  }
}

async function newClaim(api: ApiPromise, keyring: Keyring) {
  while (true) {
    console.log("\n已有账户列表：");
    accountList();
    console.log();

    let input = await new Promise<string>((resolve) => {
      rl.question("请输入账户序号(输入0退出)>: ", resolve);
    });
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。\n");
      break;
    }
    // 将 input 转换为数字
    const index = parseInt(input, 10);
    if (isNaN(index) || index < 0 || index > Key_Store.length) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }

    input = await new Promise<string>((resolve) => {
      rl.question(
        "请输入【存证内容(正整数)】(输入0退出)>: ",
        resolve
      );
    });
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。\n");
      break;
    }
    const claimData = parseInt(input, 10);
    if (isNaN(claimData) || claimData < 0) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }

    console.log("正在创建存证：");
    await createClaim(api, keyring, index, claimData);
    console.log();
  }
}

async function transferClaim(api: ApiPromise, keyring: Keyring) {
  while (true) {
    console.log("\n已有存证列表：");
    claimList();
    console.log();

    console.log("已有账户列表：");
    accountList();
    console.log();

    let input = await new Promise<string>((resolve) => {
      rl.question("请输入待转移的存证(输入0退出)>: ", resolve);
    }).catch((error) => {
        console.error(error);
        return "-1";
    });;
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。\n");
      break;
    }
    const claimIndex = parseInt(input, 10);
    if (isNaN(claimIndex) || claimIndex < 0 || claimIndex > Claim_Store.length) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }

    input = await new Promise<string>((resolve) => {
      rl.question("请输入待转入【账户序号】(输入0退出)>: ", resolve);
    }).catch((error) => {
        console.error(error);
        return "-1";
    });;
    if (input === "0") {
      console.log("已退出查看账号，回到主程序。\n");
      break;
    }
    const to = parseInt(input, 10);
    if (isNaN(to) || to < 0 || to > Key_Store.length) {
      console.log("无效的选项，请重新输入。\n");
      continue;
    }    

    console.log("正在转移存证：");
    await transferClaimAndSubscribe(api, keyring, to, claimIndex);
    console.log();
  }
}
async function removeClaim(api: ApiPromise, keyring: Keyring) {
    while (true) {
      console.log("\n已有存证列表：");
      claimList();
      console.log();
  
      let input = await new Promise<string>((resolve) => {
        rl.question("请输入待撤销的【存证序号】(输入0退出)>: ", resolve);
      }).catch((error) => {
          console.error(error);
          return "-1";
      });;
      if (input === "0") {
        console.log("已退出查看账号，回到主程序。\n");
        break;
      }
      const claimIndex = parseInt(input, 10);
      if (isNaN(claimIndex) || claimIndex < 0 || claimIndex > Claim_Store.length) {
        console.log("无效的选项，请重新输入。\n");
        continue;
      }
  
      console.log("正在撤销存证：");
      await revokeClaim(api, keyring, claimIndex);
      console.log();
    }
}

