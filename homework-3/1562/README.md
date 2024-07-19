# 安装必要工具

## 安装subxt-cli客户端

```
cargo install subxt-cli 
```
## 下载元数据

启动一条链
```
(base) daviddeMacBook-Pro:polkadot-sdk-solo-template-dev-courses david$ ./target/release/solochain-template-node --dev --tmp

```
启动链后，下载元数据
```
subxt metadata -f bytes --url ws://127.0.0.1:9944 > artifacts/polkadot_metadata_small.scale
```

# 编译
```
cargo build -r
```

# 运行

## 帮助信息

```
(base) daviddeMBP:polkadot-wallet-cli david$ ./target/release/polkadot-wallet-cli -h
Usage: polkadot-wallet-cli <COMMAND>

Commands:
  create-wallet   创建钱包
  list-wallets    列出钱包
  balance-wallet  列出钱包里的余额
  transfer        转账给其他账户
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 创建钱包

创建钱包，并查看钱包里的信息

```
(base) daviddeMBP:polkadot-wallet-cli david$ ./target/release/polkadot-wallet-cli create-wallet 
创建钱包
(base) daviddeMBP:polkadot-wallet-cli david$ cat wallet.json 

```

## 查看钱包信息
```
(base) daviddeMBP:polkadot-wallet-cli david$ ./target/release/polkadot-wallet-cli list-wallets
列出所有钱包
{
  "wallets": {
    "onblock": {
      "name": "onblock",
      "mnemonic": "ball page minor actor shallow filter tomato imitate correct goose genius parade",
      "account": {
        "address": "5Fhe1nenHptegWCSRjdgmatokgmzpaYnEi1Kci2iqWHeGSFF",
        "balance": 100000.0
      }
    },
    "daijinwei": {
      "name": "daijinwei",
      "mnemonic": "below busy pudding fork october decorate deliver lock rabbit catalog file safe",
      "account": {
        "address": "5HnGzReDmPrawXLjXjtGTYcuHAyAQHXQwbNL4FgmFyLVKaJF",
        "balance": 100000.0
      }
    }
  }
}

```
## 钱包转账

```
(base) daviddeMBP:polkadot-wallet-cli david$ ./target/release/polkadot-wallet-cli transfer --from daijinwei --to onblock --amount 100
从账户 daijinwei 转账到账户 onblock，金额 = 100
Balance transfer success: Transfer { from: AccountId32([212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125]), to: AccountId32([142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72]), amount: 100 }
(base) daviddeMBP:polkadot-wallet-cli david$ 

```