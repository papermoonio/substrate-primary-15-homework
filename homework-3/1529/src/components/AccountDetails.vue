<template>
  <div class="account-details">
    <h2>Account Details</h2>
    <p><strong>Address:</strong> {{ account.address }}</p>
    <p><strong>Mnemonic:</strong> {{ account.mnemonic }}</p>
    <p>
      <strong>Balance:</strong> {{ balance }}
      <button @click="fetchBalance">Refresh Balance</button>
    </p>
    <h3>Transfer Funds</h3>
    <div class="transfer-form">
      <label for="to">To:</label>
      <input type="text" id="to" v-model="transferTo">
      <label for="amount">Amount:</label>
      <input type="number" id="amount" v-model="transferAmount">
      <button @click="transferFunds">Transfer</button>
    </div>
    <h3>PoE Transaction</h3>
    <div class="poe-form">
      <label for="claim">Claim:</label>
      <input type="text" id="claim" v-model="poeClaim">
      <button @click="callPoETransaction">Submit PoE Claim</button>
    </div>
    <h3>Deposit Funds</h3>
    <div class="deposit-form">
      <label for="depositAmount">Amount:</label>
      <input type="number" id="depositAmount" v-model="depositAmount">
      <button @click="depositFunds">Deposit</button>
    </div>
  </div>
</template>

<script>
import api from '@/api';

export default {
  name: 'AccountDetails',
  props: {
    account: {
      type: Object,
      required: true,
    },
  },
  data() {
    return {
      balance: '',
      transferTo: '',
      transferAmount: 0,
      poeClaim: '',
      depositAmount: 0,
    };
  },
  methods: {
    async fetchBalance() {
      try {
        console.log(333, this.account.address)
        this.balance = await api.getBalance(this.account.address);
      } catch (error) {
        console.error('Error fetching balance:', error);
      }
    },
    transferFunds() {
      console.log(666,this.account)
      this.$emit('transfer', this.account, this.transferTo, this.transferAmount);
      this.transferTo = '';
      this.transferAmount = 0;
    },
    callPoETransaction() {
      this.$emit('poe', this.poeClaim);
      this.poeClaim = '';
    },
    depositFunds() {
      this.$emit('deposit', this.account.address, this.depositAmount);
      this.depositAmount = 0;
    },
  },
  created() {
    this.fetchBalance();
  },
};
</script>

<style scoped>
.account-details {
  background-color: #f0f0f0;
  padding: 20px;
  border-radius: 4px;
}

.account-details h2 {
  margin-top: 0;
}

.transfer-form,
.poe-form,
.deposit-form {
  margin-bottom: 20px;
}

label {
  display: block;
  margin-bottom: 5px;
}

input[type="text"],
input[type="number"] {
  width: 100%;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background-color: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
</style>