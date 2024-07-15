task3  命令行钱包  连接polkadot测试网络

//生成目标链元数据
cargo install subxt-cli
subxt metadata -f bytes --url ws://127.0.0.1:9944 > artifacts/polkadot_metadata_small.scale

//下载代码,并编译生成二进制可执行文件
cargo build
//查看钱包
mywallet list

xiangdong@devin:~/vsproject/substrate-primary-15-homework/homework-3/1525/mywallet$ target/debug/mywallet list
Client successfully connected to ws://127.0.0.1:9944
account:0
address:5DSJTshx2FW6JURodVQNRWKVXJ3qdq2yR6229nAKmCUego3B
mnemonic:skill bread large choose rich host chief must doll tuition funny squeeze


//创建钱包
mywallet generate

xiangdong@devin:~/vsproject/substrate-primary-15-homework/homework-3/1525/mywallet$ target/debug/mywallet generate
Client successfully connected to ws://127.0.0.1:9944
mnemonic:legend three advice portion choose distance prosper stuff saddle evoke furnace rose
address:5GH3QTMGKb9Z4gwhdKaZCfqx9erfXB8FcfyYg5NsGseYVXW5

//导入钱包
mywallet import --mnemonic  

xiangdong@devin:~/vsproject/substrate-primary-15-homework/homework-3/1525/mywallet$ target/debug/mywallet import --mnemonic "legend three advice portion choose distance prosper stuff saddle evoke furnace rose"
Client successfully connected to ws://127.0.0.1:9944
wallet is exists

//从内置账户alice获取初始资金, 需要用到索引
mywallet init-amount --number 

xiangdong@devin:~/vsproject/substrate-primary-15-homework/homework-3/1525/mywallet$ target/debug/mywallet init-amount --number 0
Client successfully connected to ws://127.0.0.1:9944
balance transfer success:Transfer { from: AccountId32([212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125]), to: AccountId32([60, 180, 226, 111, 47, 180, 82, 65, 209, 63, 5, 102, 219, 149, 9, 204, 179, 85, 78, 104, 144, 74, 100, 208, 241, 143, 70, 129, 41, 76, 97, 33]), amount: 1000000000000 }



//交易转账
mywallet transfer --number --to --amount

xiangdong@devin:~/vsproject/substrate-primary-15-homework/homework-3/1525/mywallet$ target/debug/mywallet transfer --number 0 --to 5GH3QTMGKb9Z4gwhdKaZCfqx9erfXB8FcfyYg5NsGseYVXW5  --amount 50000000
Client successfully connected to ws://127.0.0.1:9944
balance transfer success:Transfer { from: AccountId32([60, 180, 226, 111, 47, 180, 82, 65, 209, 63, 5, 102, 219, 149, 9, 204, 179, 85, 78, 104, 144, 74, 100, 208, 241, 143, 70, 129, 41, 76, 97, 33]), to: AccountId32([186, 89, 174, 232, 131, 105, 182, 51, 193, 17, 166, 102, 212, 31, 0, 34, 110, 255, 84, 195, 5, 95, 89, 236, 5, 76, 178, 157, 38, 217, 229, 120]), amount: 50000000 }

//查询余额
mywallet balance --address

xiangdong@devin:~/vsproject/substrate-primary-15-homework/homework-3/1525/mywallet$ target/debug/mywallet balance --address 5GH3QTMGKb9Z4gwhdKaZCfqx9erfXB8FcfyYg5NsGseYVXW5
Client successfully connected to ws://127.0.0.1:9944
Account balance: 50000000

