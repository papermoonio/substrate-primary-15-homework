1. 创建账户命令
```
node index.js create-account
```
```
Mnemonic: chat spray dwarf oval unique fault shuffle good wet bracket law glance
Address: 5E66A9vWvjSxwXeK7ZvpJLVjGbxPKmoKnEAF8HEzMaw3sLu8
```

2. 查询账户余额
```
node index.js check-balance 5E66A9vWvjSxwXeK7ZvpJLVjGbxPKmoKnEAF8HEzMaw3sLu8
```
```
Balance of 5E66A9vWvjSxwXeK7ZvpJLVjGbxPKmoKnEAF8HEzMaw3sLu8: 0
```

3. 转账到其他帐号
```
node index.js transfer 5EvuyBfdu2vXp5C24RXkgPVCcaYu2g7obHfx3MGf4E5vkdPB 5E66A9vWvjSxwXeK7ZvpJLVjGbxPKmoKnEAF8HEzMaw3sLu8 1
```
```
...
```

4. 入账提醒
```
node index.js monitor-balance 5E66A9vWvjSxwXeK7ZvpJLVjGbxPKmoKnEAF8HEzMaw3sLu8
```