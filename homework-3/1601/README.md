# hw3-1601

## Task Report:
1. 可以新建帐号，查看余额
* 1.1 去到創建或加入錢包的頁面
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/1_1.png?raw=true)
* 1.2 創建新錢包
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/1_2.png?raw=true)
* 1.3 看到新錢包地址
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/1_3.png?raw=true)
* 1.4 查看當下餘額
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/1_4.png?raw=true)
2. 显示钱包地址
* 2.1 可以在登入後顯示錢包完整地址
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/2_1.png?raw=true)
3. 转账到其他帐号
* 3.1 首先去 https://polkadot.js.org/apps/#/accounts
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/3_1.png?raw=true)
* 3.2 利用開發模式中的戶口轉錢到 5E1eMotU6YAnv3y7EHSVKtzszrKCPz1ZtmjkpZeSevjoLRNM
然後由 5E1eMotU6YAnv3y7EHSVKtzszrKCPz1ZtmjkpZeSevjoLRNM 轉到
5DkdPxqT8vvfEHzbhAkmg3cQ3ZYYGyyUSVoT8LDmrVDUyagm
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/3_2.png?raw=true)
* 3.3 確認交易
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/3_3.png?raw=true)
4. 入账提醒
* 4.1 當接受到款項，會在頁面出現通知
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/4_1.png?raw=true)
5. 调用特定Substrate链的交易 （PoE链）
* 5.1 輸入資料的頁面
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/5_1.png?raw=true)
* 5.2 完成後的交易紀錄
![alt text](https://github.com/MartinYeung5/substrate-primary-15-homework/blob/main/homework-3/1601/public/5_2.png?raw=true)
6. 参考知名钱包的任何功能 （可选）

## Faucet
westend faucet:
https://faucet.polkadot.io/westend

## RPC endpoints:
https://substrate.rs/overview-westend-wnd.html

### Bug fixed:
1. Argument of type 'string' is not assignable to parameter of type 'Uint8Array'.ts(2345)
* Solution: 
const encoder = new TextEncoder();
const uintArrayValue = encoder.encode("String");