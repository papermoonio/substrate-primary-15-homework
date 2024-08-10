import {ApiPromise,WsProvider,Keyring} from "@polkadot/api";
import { account } from "@polkadot/api-derive/balances";
import { Header } from "@polkadot/types/interfaces";
import { balances } from "@polkadot/types/interfaces/definitions";
import { bnToBn, objectEntries } from "@polkadot/util";
import { resolve } from "path";

const local_url = "ws://127.0.0.1:9944";
function delay(ms:number){
    return new Promise((resolve)=>setTimeout(resolve,ms));
}
const main =async()=>{
    // 初始化api
    const wsProvider = new WsProvider(local_url);
    const api=await ApiPromise.create({
        provider :wsProvider,
        types:{},
    });
    // 等待事件
    await api.isReady;
    
    const ADDR1="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    const ADDR2="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
     
    // 1.创建账户，查看余额
    const keyring = new Keyring({type:"sr25519"});
     const alice = keyring.addFromUri("//Alice");
     const bob = keyring.addFromUri("//Bob");
    const tom = keyring.addFromUri("//Tom");
    const ADDR3=tom.address;
     await api.query.system.account( ADDR1,(account:{ nonce:number;data:{ free:string; };})=>{
                console.log(
                    `current nonce is ${account.nonce},blance is ${account.data.free}`,
                ); },
    );
    await delay(100);
    // 2.显示钱包地址
    console.log(`tom's address is ${ADDR3}`);
    // 3.转账到其他账号
    const tx=api.tx.balances.transferKeepAlive(ADDR3,12345);
    const hash=await tx.signAndSend(alice);
    console.log(`Add stake transcation sent with hash ${hash.toHex()}`);
    // 4.入账提醒
    await api.query.system.account(
        // 下面是先解析account
        ADDR1,
        (
            account:{
                nonce:number;
                data:{
                    free:string;
                };
            })=>{
                console.log(
                    `current nonce is ${account.nonce},blance is ${account.data.free}`,
                );
            },
    );
    await delay(20000);
    // 5.调用poe链的交易
    const myByte: Uint8Array = new Uint8Array([0x02]);
    const result=await api.tx.poeModule.createClaim(4).signAndSend(bob);
    console.log(`createClaim hash: ${result.toHex()}`);

    // 6.其他：
   
};

main()
    .then(()=>{
        console.log("successfully exited!");
        process.exit(0);
    })
    .catch((err)=>{
        console.log("error occur:",err);
        process.exit(1);
    })