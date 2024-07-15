import React, { useState, useEffect } from 'react';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
const local_url = "ws://127.0.0.1:9944";

const style1 = {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'flex-start'
}
function Wallet() {
    const [api, setApi] = useState(null);
    const [account, setAccount] = useState(null);
    const [balance, setBalance] = useState('');
    const [balance1, setBalance1] = useState('');
    const [addressInput, setAddressInput] = useState('');
    const [accountAddress, setAccountAddress] = useState('');
    const [mnemonicInput, setMnemonicInput] = useState('');
    const [mnemonicPhrase, setMnemonicPhrase] = useState('');

    const [transferAmount, setTransferAmount] = useState('');

    useEffect(() => {
        initApi()
            .then(() => {
                console.log('成功')
            })
            .catch((error) => {
                console.log('失败', error)
            });
    }, []);

    async function initApi() {
        const provider = new WsProvider(local_url);
        const api = await ApiPromise.create({ provider, types: {} });
        setApi(api);
    }
    // create a account
    async function createAccount() {
        if (!mnemonicInput.trim()) {
            alert('请输入助记词!');
            return;
        }

        const keyring = new Keyring({ type: 'sr25519' });
        const account = keyring.addFromUri(mnemonicInput);
        // setAccount(account)
        setAccountAddress(account.address);
        setMnemonicPhrase(account.phrase);

        // Check balance for the new account
        const { data: { free } } = await api.query.system.account(account.address);
        setBalance(free.toHuman());
    }

    // get balance
    async function getBalance(address) {
        // console.log(api)
        if (!api) {
            alert('请初始化API');
            return
        }
        if (address === '') {
            alert('输入地址才能查余额');
            return
        }
        const ADD1 = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
        const { data: { free } } = await api.query.system.account(address);
        // console.log(free)
        console.log(free.toHuman())

        setBalance1(free.toHuman());
    }
    async function transferFunds() {
        const keyring = new Keyring({ type: 'sr25519' });


        // 从 URI '//Alice' 生成 Alice 的密钥对
        const alice = keyring.addFromUri('//Alice')
        const bobAdd = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
        // 目标地址和金额
        const tx = api.tx.balances.transferKeepAlive(bobAdd, Number(transferAmount))
        const hash = await tx.signAndSend(alice)

        console.log('Transfer sent with hash', hash.toHex());

    }
    async function transfer() {
        const keyring = new Keyring({ type: 'sr25519' });


        // 从 URI '//Alice' 生成 Alice 的密钥对
        const alice = keyring.addFromUri('//Alice')
        const tx = api.tx.whxModule.createClaim(0x1)
        const hash = await tx.signAndSend(alice)

        console.log('Transfer sent with hash', hash.toHex());

    }
    return (
        <div>
            {/* <button onClick={initApi}>初始化API</button> */}

            <button onClick={createAccount}>新建账户</button>
            <input
                type="text"
                value={mnemonicInput}
                onChange={(e) => setMnemonicInput(e.target.value)}
                placeholder="输入助记词"
            />
            {accountAddress && (
                <div style={style1}>
                    <p>钱包地址: {accountAddress}</p>
                    <p>助记词: {mnemonicInput}</p>
                    <p>余额: {balance !== null ? balance : 'Loading...'}</p>
                </div>
            )}
            <button onClick={() => getBalance(addressInput)}>查余额</button>
            <input
                type="text"
                value={addressInput}
                onChange={(e) => setAddressInput(e.target.value)}
                placeholder="Enter address"
            />
            {addressInput &&
                <div style={style1}>

                    <p>钱包地址: {addressInput}</p>
                    <p>钱包余额: {balance1}</p>
                </div>
            }
            <h2>转账</h2>
            <div style={style1}>
                从Alice给Bob转账测试
                <input
                    type="number"
                    value={transferAmount}
                    onChange={(e) => setTransferAmount(e.target.value)}
                    placeholder="输入转账金额"
                />
                <button onClick={() => transferFunds()}>转账</button>
            </div>
            <h2>交易</h2>
            <p> 测试poe链whxModule模块createClaim功能</p>
            <button onClick={() => transfer()}>交易</button>
        </div>
    );
}

export default Wallet;
