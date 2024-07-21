import { useState, useEffect } from 'react';
import { notification, message } from 'antd';
import { newAccount, queryBalances, execTransfer, addClaim } from '../services/user';

const usePolkadot = () => {
    const [name, setName] = useState('test');
    const [loading, setLoading] = useState(false);
    const [mnemonic, setMnemonic] =  useState('');
    const [account, setAccount] = useState({});
    const [address, setAddress] = useState('');
    const [balances, setBalances] = useState(0);
    const [to, setTo] = useState('');
    const [amount, setAmount] = useState(0);
    const [claim, setClaim] = useState('');
    const createAccount = async () => {
        setLoading(true);
        newAccount(mnemonic).then((res) => {
            const { address } = res;
            setAccount(address);
            notification.open({
                duration: null,
                message: '账号创建成功',
                description: `账号地址为：${address}`,
              });
              setLoading(false);
        });
    };
    const getBalances = async () => {
        setLoading(true);
        queryBalances(address).then((num) => {
            setBalances(num);
            setLoading(false);
        });
    }

    const transfer = async () => {
        execTransfer(to, amount).then((res) => {
            message.success('转账成功,转账hash为：' + res);
        });
    }

    const createClaim = async () => {
        addClaim(claim).then((res) => {
            console.log('存证hash为：' + res);
            message.success('存证hash为：' + res);
        })
    }
    
    return {
        name,
        setName,
        loading,
        setLoading,
        mnemonic,
        setMnemonic,
        createAccount,
        account,
        address,
        setAddress,
        balances,
        getBalances,
        to,
        setTo,
        transfer,
        amount,
        setAmount,
        claim,
        setClaim,
        createClaim
    };
}

export default usePolkadot;