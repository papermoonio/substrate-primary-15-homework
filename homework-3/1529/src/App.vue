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
      <account-details v-if="selectedAccount" :account="selectedAccount" @transfer="handleTransfer" @poe="handlePoETransaction" @deposit="handleDeposit"></account-details>
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

        // 转账成功后的逻辑，例如更新账户余额、显示成功消息等
        // ...

      } catch (error) {
        console.error('Error transferring funds:', error);
        // 处理转账错误，例如显示错误消息
        // ...
      }
    },  
    async fetchSelectedAccountBalance() {
      if (this.selectedAccount) {
        try {
          console.log(222,this.selectedAccount.address)
          this.selectedAccount.balance = await api.getBalance(this.selectedAccount.address);
        } catch (error) {
          console.error('Error fetching balance:', error);
        }
      }
    },
    async handlePoETransaction(form, claim) {
      try {
        await api.callPoETransaction(form, claim);
      } catch (error) {
        console.error('Error calling PoE transaction:', error);
      }
    },
    async handleDeposit(to, amount) {
      try {
        await api.depositFunds(to, amount); 
        await this.fetchSelectedAccountBalance();
        // ...
      } catch (error) {
        console.error('Error depositing funds:', error); 
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
  position: fixed;
  top: 0;
  right: 0;
  width: 300px;
  height: 100%;
  background-color: #f0f0f0;
  padding: 20px;
  overflow-y: auto;
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