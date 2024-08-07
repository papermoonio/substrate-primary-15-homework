<template>
  <div class="account-details">
    <h2>Account Details</h2>
    <p>Address: {{ account.address }}</p>
    <p>Mnemonic: {{ account.mnemonic }}</p>
    <p>Balance: {{ account.balance }}</p>
    <div class="transfer-section">
      <h3>Transfer Funds</h3>
      <input v-model="transferTo" placeholder="Recipient Address" />
      <input v-model="transferAmount" placeholder="Amount" />
      <button @click="transfer">Transfer</button>
    </div>
    <div class="poe-section">
      <h3>PoE Transaction</h3>
      <input v-model="poeClaim" placeholder="Claim" />
      <button @click="callPoe">Submit Claim</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'AccountDetails',
  props: ['account'],
  data() {
    return {
      transferTo: '',
      transferAmount: '',
      poeClaim: '',
    };
  },
  methods: {
    transfer() {
      this.$emit('transfer', this.account.address, this.transferTo, this.transferAmount);
      this.transferTo = '';
      this.transferAmount = '';
    },
    callPoe() {
      this.$emit('poe', this.poeClaim);
      this.poeClaim = '';
    },
  },
};
</script>