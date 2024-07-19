import React, { useEffect, useState } from 'react';
import { createAccount, getBalance, transfer, subscribeToBalanceChanges, callPoEChainTransaction } from './polkadot';
import './App.css';


function App() {
  const [account, setAccount] = useState(null);
  const [balance, setBalance] = useState(null);
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [status, setStatus] = useState('');
  const [hashCode, setHashCode] = useState(null);

  const handleCreateAccount = async () => {
    const { pair, mnemonic } = await createAccount();
    setAccount(pair);
    alert(`New account created. Address: ${pair.address}\nMnemonic: ${mnemonic}`);
  };

  const handleGetBalance = async () => {
    alert(`Get account balance : ${balance}`);
  };

  useEffect(() => {
    if (account) {
      const fetchBalance = async () => {
        const bal = await getBalance(account.address);
        setBalance(bal);
        subscribeToBalanceChanges(account.address, (newBalance) => {
          setBalance(newBalance);
          alert(`New balance: ${newBalance}`);
        });
      };
      fetchBalance();
    }
  }, [account]);

  const handleTransfer = async () => {
    try {
      setStatus('Transferring...');
      console.log("{account}",account, recipient, amount);
      const hash = await transfer(account, recipient, amount);
      setStatus(`Transfer successful with hash: ${hash}`);
      const bal = await getBalance(account.address);
      setBalance(bal);
    } catch (error) {
      setStatus(`Transfer failed: ${error.message}`);
    }
  };

  const handlePoEChainTransaction = async () => {
    try {
      setStatus('Calling PoE chain transaction...');
      const hashInfo = await callPoEChainTransaction(hashCode);
      setStatus(`PoE chain blockhash: ${hashInfo}`);
    } catch (error) {
      setStatus(`PoE chain blockhash get failed: ${error.message}`);
    }
  };

  return (
    <div className="container">
      <h1>Polkadot.js Practice - Homework 3</h1>
      <button onClick={handleCreateAccount}>Create New Wallet</button>
      {account && (
        <div>
          <p>Account Address: {account.address}</p>
        </div>
      )}
      <button onClick={handleGetBalance}>Get Balance</button>
      {balance && (
        <div>
          <p>Balance: {balance}</p>
        </div>
      )}
      <div>
        <input
          type="text"
          placeholder="Recipient Address"
          value={recipient}
          onChange={(e) => setRecipient(e.target.value)}
        />
        <input
          type="text"
          placeholder="Sender Value"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
        <button onClick={handleTransfer}>Transfer</button>
      </div>
      <div>
        <input
          type="text"
          placeholder="Block Number"
          value={hashCode}
          onChange={(e) => setHashCode(e.target.value)}
        />
        <button onClick={handlePoEChainTransaction}>Call Poe chain get blockHash</button>
      </div>
      {status && <p>{status}</p>}
    </div>
  );
}

export default App;
