<template>
  <div id="app">
    <div class="sidebar">
      <h2>Accounts</h2>
      <button @click="createAccount">Create Account</button>
      <ul>
        <li v-for="(account, index) in accounts" :key="index" @click="selectAccount(account)">
          {{ account.name }}
        </li>
      </ul>
    </div>
    <div class="main-content">
      <account-details v-if="selectedAccount" :account="selectedAccount" @transfer="handleTransfer" @poe="handlePoETransaction"></account-details>
    </div>
    <div class="activity-sidebar">
      <h2>Account Activity</h2>
      <ul>
        <li v-for="(activity, index) in accountActivity" :key="index">
          {{ activity }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
import AccountDetails from './components/AccountDetails.vue';
import api from './api';

export default {
  name: 'App',
  components: {
    AccountDetails,
  },
  data() {
    return {
      accounts: [],
      selectedAccount: null,
      accountActivity: [],
    };
  },
  methods: {
    async createAccount() {
      try {
        const newAccount = await api.createAccount();
        this.accounts.push({
          name: `Account ${this.accounts.length + 1}`,
          ...newAccount,
        });
      } catch (error) {
        console.error('Error creating account:', error);
      }
    },
    selectAccount(account) {
      this.selectedAccount = account;
    },
    async handleTransfer(from, to, amount) {
      try {
        await api.transferFunds(from, to, amount);
      } catch (error) {
        console.error('Error transferring funds:', error);
      }
    },
    async handlePoETransaction(claim) {
      try {
        await api.callPoETransaction(claim);
      } catch (error) {
        console.error('Error calling PoE transaction:', error);
      }
    },
  },
  mounted() {
    api.init().then(() => {
      console.log('Connected to Polkadot network');
      api.listenForAccountActivity((activity) => {
        this.accountActivity.push(activity);
      });
    }).catch((error) => {
      console.error('Error connecting to Polkadot network:', error);
    });
  },
};
</script>

<style>
#app {
  display: flex;
  height: 100vh;
}

.sidebar {
  width: 200px;
  background-color: #f0f0f0;
  padding: 20px;
}

.sidebar h2 {
  margin-top: 0;
}

.sidebar ul {
  list-style-type: none;
  padding: 0;
}

.sidebar li {
  cursor: pointer;
  margin-bottom: 10px;
}

.main-content {
  flex: 1;
  padding: 20px;
}

.activity-sidebar {
  width: 300px;
  background-color: #f0f0f0;
  padding: 20px;
}

.activity-sidebar h2 {
  margin-top: 0;
}

.activity-sidebar ul {
  list-style-type: none;
  padding: 0;
}

.activity-sidebar li {
  margin-bottom: 10px;
}
</style>