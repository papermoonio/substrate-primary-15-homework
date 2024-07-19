import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ALICE, ALICE_URI, BOB, BOB_URI, Key_Store, WS_URL } from "../constants/constants";

export const sleep = async (ms: number) => {
  await new Promise((resolve) => setTimeout(resolve, ms));
};

export const connect = async () => {
  const wsProvider = new WsProvider(WS_URL);
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {},
  });
  await api.isReady;
  return api;
};

// export const injectMetadata = async (api: ApiPromise) => {
//   // 获取当前的 metadata
//   const metadata = await api.rpc.state.getMetadata();

//   // 获取当前的 runtime 版本
//   const runtimeVersion = await api.runtimeVersion;

//   // 注入新的 metadata
//   api.injectMetadata(metadata, true);

//   console.log("Metadata 已更新");
// };

export const readRuntimeVersion = async (api: ApiPromise) => {
  const runtimeVersion = await api.runtimeVersion;
  console.log("runtimeVersion.specVersion:", runtimeVersion.specVersion.toNumber());
};

/** 生成Keyring，并将Alice 和 Bob 添加到STORES中 */
export const createKeyring = () => {
    const keyring  = new Keyring({ type: 'sr25519' });
    keyring.addFromUri(ALICE_URI, { name: ALICE});
    keyring.addFromUri(BOB_URI, { name: BOB});

    Key_Store.push(keyring.pairs[0]);
    Key_Store.push(keyring.pairs[1]);
    return keyring;
}

// 检查字符串是否为有效的16进制数
export function isValidHex(input: string): boolean {
    return /^0x[0-9a-fA-F]+$/i.test(input);
}

// 将16进制字符串转换为Uint8Array
export function hexToUint8Array(hexString: string): Uint8Array {
    // 移除前缀"0x"，然后每两个字符分割
    const hex = hexString.slice(2);
    const uint8Array = new Uint8Array(hex.length / 2);
    for (let i = 0; i < uint8Array.length; i++) {
        uint8Array[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
    }
    return uint8Array;
}

let printStarFlag = true;
export function setPrintStarFlag(value: boolean) {
    printStarFlag = value;
}

export function getPrintStarFlag() {
    return printStarFlag;
}

/** 打印星号 */
export const printStar = () => {
    if (printStarFlag) {
        process.stdout.write("*");
        setTimeout(printStar, 500);
    }
}

// 示例使用
// const input = "0x1a2b3c4d";
// if (isValidHex(input)) {
//     const uint8Array = hexToUint8Array(input);
//     console.log("转换成功，Uint8Array:", uint8Array);
// } else {
//     console.log("输入的不是有效的16进制数。");
// }


