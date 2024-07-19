import { ApiPromise, Keyring } from "@polkadot/api";
import { mnemonicGenerate } from "@polkadot/util-crypto";
import { ALICE_URI, BOB_URI, Key_Store } from "../constants/constants";
import Account from "../models/account";
import { printStar, setPrintStarFlag } from "../utils/utils";

/** 生成助记词 */
export const generateMnemonic = () => {
    const mnemonic = mnemonicGenerate(12);
    return mnemonic;
}

/** 创建账户 */
export const createAccount = (keyring: Keyring, mnemonic: string, username: string) => {
    const account = keyring.addFromUri(mnemonic, { name: username });
    Key_Store.push(account);
    return account;
}

/** 选择账户 */ 
export const accountList = () => {
    Key_Store.forEach((account, index) => {
        console.log(`${index + 1}. ${account.meta.name}`);
    });
    // return account;
}

/** 读取账户 */ 
export const readAccount = async (api: ApiPromise, index: number) => {
    const address = Key_Store[index - 1].address;
    const queryResult = await api.query.system.account(address);

    const queryResultJson = JSON.stringify(queryResult.toHuman());
    const freeBalance = JSON.parse(queryResultJson).data.free;
    const account = new Account(Key_Store[index - 1].meta.name, address, freeBalance);
    return account;
}

/**
 * 转账并订阅到账提醒
*/
export const transferAndSubscribe = (api: ApiPromise, keyring: Keyring, 
    from: number, to: number, amount: number) => {

    return new Promise(async (resolve, reject) => {
        setPrintStarFlag(true);
        printStar();

        const fromAccount = Key_Store[from - 1];
        const toAccount = Key_Store[to - 1];

        try {
            const tx = await api.tx.balances.transferKeepAlive(toAccount.address, amount);
            await tx.signAndSend(fromAccount, ({events = [], status}) => {
                if (status.isFinalized) {
                    setPrintStarFlag(false);
                    console.log(`\n从 ${fromAccount.meta.name} 到 ${toAccount.meta.name} 的转账成功。\n交易状态哈希：${status.hash.toHex()}`);
                    resolve(void 0);    // resolve：当Promise成功完成时调用此函数。它将Promise的状态从“待定”（pending）更改为“已完成”（fulfilled）。resolve函数接受一个参数，这个参数是Promise成功完成时的返回值。如果resolve被调用，那么与该Promise相关联的.then回调会被执行。
                }
            });
        } catch (error) {
            reject(error);  // reject：当Promise因为错误或其他原因失败时调用此函数。它将Promise的状态从“待定”（pending）更改为“已拒绝”（rejected）。reject函数接受一个参数，这个参数通常是一个错误对象，说明Promise为什么会被拒绝。如果reject被调用，那么与该Promise相关联的.catch回调会被执行。          
        }        
    });
}