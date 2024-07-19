import { KeyringPair } from "@polkadot/keyring/types";
import Claim from "../models/Claim";

export const WS_URL = "ws://127.0.0.1:9944";
export const ALICE_ADDRESS = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
export const BOB_ADDRESS = "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc";
export const ALICE_URI = "//Alice";
export const BOB_URI = "//Bob";
export const ALICE = "Alice";
export const BOB = "Bob";

// 由于 SR25519 密钥对不会存储在 keyring 中，所以暂且存储到一个全局变量（不能用在生产环境！）
export const Key_Store: KeyringPair[] = [];
// 存证存储
export const Claim_Store: Claim[] = [];

// export const Max_Claim_Length = 3;