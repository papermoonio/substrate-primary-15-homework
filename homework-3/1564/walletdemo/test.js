import { createAccount, createApi, getAddress, getBalance } from "./src/apis/index.js";

const account = await createAccount();

console.log(account.mnemonic);
console.log(account.pair.address);

console.log(getAddress(account.pair, 0));
console.log(getAddress(account.pair, 1));
console.log(getAddress(account.pair, 2));
console.log(getAddress(account.pair, 3));
console.log(getAddress(account.pair, 4));
console.log(getAddress(account.pair, 5));
console.log(getAddress(account.pair, 137));


const api = await createApi("varatest");


getBalance(api,"kGfxF2ew9eSRnSAdj92xzW2kFXa47MX6g9vT5aZQr2ehe56T5").then(val=>{
    console.log(val);
});getBalance(api,"EXtQYFeY2ivDsfazZvGC9aG87DxnhWH2f9kjUUq2pXTZKF5").then(val=>{
    console.log(val);
});

