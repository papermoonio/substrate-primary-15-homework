import { ApiPromise, Keyring } from "@polkadot/api";
import { Claim_Store, Key_Store } from "../constants/constants";
import { printStar, setPrintStarFlag } from "../utils/utils";
import Claim from "../models/Claim";

export const createClaim = async (api: ApiPromise, keyring: Keyring, index: number, claimData: number) => {
    const sender = Key_Store[index - 1];
    console.log(`sender: ${sender.meta.name} - ${sender.address}`);
    
    return new Promise(async (resolve, reject) => {
        setPrintStarFlag(true);
        printStar();
        try {
            console.log(`claimData: ${claimData}`);
            const tx = await api.tx.poeModule.createClaim(claimData);
            // const tx = await api.tx.templateModule.doSomething(1000);
            const hash = await tx.signAndSend(sender);
            setPrintStarFlag(false);
            let name = sender.meta.name;
            if (name === undefined) {
                name = "Unknown";
            }
            const claim = new Claim(claimData, name, sender.address);
            Claim_Store.push(claim);
            console.log(`\n存证创建成功，交易哈希：${hash.toHex()}`);
            resolve(void 0);
        } catch (error) {
            reject(error);            
        }
    });
}

export const claimList = () => {
    Claim_Store.forEach((claim, index) => {
        console.log(`${index + 1}. ${claim.claimData} - ${claim.owner}`);
    });
}

/** 转移存证并订阅消息*/
export const transferClaimAndSubscribe = (api: ApiPromise, keyring: Keyring, to: number, claimIndex: number) => {
    const claim = Claim_Store[claimIndex - 1];
    console.log(`claim: ${claim.claimData} - ${claim.owner}`);
    const sender = Key_Store.filter((account) => account.address === claim.address)[0];
    console.log(`sender: ${sender.meta.name} - ${sender.address}`);
    const receiver = Key_Store[to - 1];
    console.log(`receiver: ${receiver.meta.name} - ${receiver.address}`);
    
    return new Promise(async (resolve, reject) => {
        setPrintStarFlag(true);
        printStar();
        try {
            const tx = await api.tx.poeModule.transferClaim(receiver.address, claim.claimData);
            await tx.signAndSend(sender, ({events = [], status}) => {
                if (status.isFinalized) {
                    setPrintStarFlag(false);
                    // 更新Claim_Store
                    let name = receiver.meta.name;
                    if (name === undefined) {
                        name = "Unknown";
                    }
                    claim.owner = name;
                    claim.address = receiver.address;
                    console.log(`\n存证转移成功，交易状态哈希：${status.hash.toHex()}`);
                    resolve(void 0);
                }
            });
        } catch (error) {
            reject(error);
        }
    });
}

/** 撤销存证 */
export const revokeClaim = (api: ApiPromise, keyring: Keyring, index: number) => {
    const claim = Claim_Store[index - 1];
    console.log(`claim: ${claim.claimData} - ${claim.owner}`);
    const sender = Key_Store.filter((account) => account.address === claim.address)[0];
    console.log(`sender: ${sender.meta.name} - ${sender.address}`);
    
    return new Promise(async (resolve, reject) => {
        setPrintStarFlag(true);
        printStar();
        try {
            const tx = await api.tx.poeModule.revokeClaim(claim.claimData);
            await tx.signAndSend(sender, ({events = [], status}) => {
                if (status.isFinalized) {
                    setPrintStarFlag(false);
                    // 从Claim_Store中删除
                    const index = Claim_Store.findIndex((claim) => claim.claimData === claim.claimData);
                    Claim_Store.splice(index, 1);
                    console.log(`\n存证撤销成功，交易状态哈希：${status.hash.toHex()}`);
                    resolve(void 0);
                }
            });
        } catch (error) {
            reject(error);
        }
    });
}