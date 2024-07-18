<template>
  <div class="popup-wrap">
    <div v-if="loading" class="setup-loading">Loading...</div>

    <div class="setup-steps">
      <div v-if="step === 'first'" class="setup-box setup-box1">
        <h3>Welcome to MyWallet</h3>
        <div class="btns">
          <button @click="createWallet" class="btn">Create Wallet</button>
          <button @click="importWallet" class="btn">Import Wallet</button>
        </div>
      </div>

      <div v-if="step === 'second_create'" class="setup-box setup-box2">
        <h3>Create Wallet</h3>
        <div>
          <label for="wordslen">Select Words Length:</label>
          <select
            v-model="wordslen"
            @change="regenerate"
            id="wordslen"
            class="select">
            <option value="12">12</option>
            <option value="15">15</option>
            <option value="18">18</option>
            <option value="21">21</option>
            <option value="24">24</option>
          </select>
        </div>
        <div class="word-list">{{ newwords }}</div>
        <div class="btns">
          <button @click="confirmCreate" class="btn">Confirm</button>
        </div>
      </div>

      <div v-if="step === 'second_import'" class="setup-box setup-box2">
        <h3>Import Wallet</h3>
        <div class="word-list">
          <textarea
            v-model="importwords"
            class="input"
            placeholder="Enter mnemonic words"
            style="height: 180px"></textarea>
        </div>
        <div class="errormsg">{{ errmsg }}</div>
        <div class="btns">
          <button @click="confirmImport" class="btn">Confirm</button>
        </div>
      </div>

      <!-- <div v-if="step === 'third_setpass'" class="setup-box setup-box3">
        <h3>Set Password</h3>
        <div class="input-pass">
          <input v-model="password" class="input" type="password" placeholder="Enter password" />
        </div>
        <div class="errormsg">{{ errmsg }}</div>
        <div class="btns">
          <button @click="confirmPass" class="btn">Confirm</button>
        </div>
      </div> -->

      <div v-if="step === 'four_success'" class="setup-box setup-box4">
        <h3>Result</h3>
        <div class="input-pass">Succeeded</div>
        <div class="btns">
          <button @click="toStart" class="btn">Start</button>
        </div>
      </div>
    </div>

    <div>
      <h3>accounts:</h3>
      <div class="accounts-list">
        <select
          v-model="selectedAcc"
          @change="changeSelect"
          id="account-select"
          class="select">
          <option value="">请选择你的账号</option>
          <option v-for="item in accounts" :key="item.name" :value="item.name">
            {{ item.name }}
          </option>
        </select>
      </div>
      <div class="address-list">
        <ul>
          <li v-for="(item, i) in addressList" :key="item.address">
            <div class="layer1">
              <div class="info">
                {{ i + 1 }}: {{ item.address }} - {{ item.symbol }} -
                {{ item.network }}
              </div>
            </div>
            <div class="layer2">
              <span>余额:{{ item.balance || 0 }}</span>
              <div class="links">
                <button class="btn" @click="showBalance(item)">查询余额</button>
              </div>
            </div>
            <div class="layer3">
              <label for="">地址</label>
              <input
                type="text"
                style="width: 460px"
                placeholder="目标地址"
                v-model="item.toaddress" />
            </div>
            <div class="layer3">
              <label for="">金额</label>
              <input
                type="text"
                style="width: 460px"
                placeholder="金额"
                v-model="item.toamount" />
            </div>

           <div class="layer4">
            <button class="btn" @click="toTransfer(item)">转账</button>
           </div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
import { walletContext } from "../context";
// import registryJson from "@substrate/ss58-registry";
import "./style.scss";

const registryJson = [
  {
    prefix: 0,
    network: "polkadot",
    displayName: "Polkadot Relay Chain",
    symbols: ["DOT"],
    decimals: [10],
    standardAccount: "*25519",
    website: "https://polkadot.network",
  },
  {
    prefix: 2,
    network: "kusama",
    displayName: "Kusama Relay Chain",
    symbols: ["KSM"],
    decimals: [12],
    standardAccount: "*25519",
    website: "https://kusama.network",
  },
  {
    prefix: 137,
    network: "varatest",
    displayName: "Vara Test Network",
    symbols: ["VARA"],
    decimals: [12],
    standardAccount: "*25519",
    website: "https://vara.network/",
  },
];

export default {
  data() {
    return {
      loading: true,
      accounts: [],
      step: "first",
      newwords: "",
      wordslen: "12",
      importwords: "",
      password: "",
      errmsg: "",
      selectedAcc: "",
      addressList: [],
      validRegistrys: registryJson.filter((temp) => temp.symbols.length > 0),
    };
  },
  computed: {
    accountlen() {
      return this.accounts.length;
    },
  },
  mounted() {
    this.loadAccounts();
    console.log("Hello from the popup!");
    console.log("registryJson===", registryJson);
  },
  methods: {
    loadAccounts() {
      return walletContext.loadData().then(() => {
        this.accounts = [];
        const accounts = walletContext.wallets;
        const keys = this.accounts.map((p) => p.name);
        accounts.forEach((temp) => {
          if (keys.indexOf(temp.name) == -1) {
            this.accounts.push(temp);
          }
        });
        this.loading = false;
        if (this.accounts.length > 0) {
          this.selectedAcc = this.accounts[0].name;
          this.changeSelect();
        }
      });
    },
    async createWallet() {
      const account = await walletContext.generateWallet(12);
      this.newwords = account.mnemonic;
      this.step = "second_create";
    },
    async regenerate() {
      const account = await walletContext.generateWallet(
        parseInt(this.wordslen)
      );
      this.newwords = account.mnemonic;
    },
    importWallet() {
      this.step = "second_import";
    },
    confirmCreate() {
      this.step = "four_success";
    },
    confirmImport() {
      const len = this.importwords.split(" ").length;
      if (![12, 15, 18, 21, 24].includes(len)) {
        this.errmsg = "Invalid word length";
        return;
      }
      walletContext.importWallet(this.importwords);
      this.step = "four_success";
    },
    // confirmPass() {
    //   this.step = "four_success";
    // },
    toStart() {
      this.loadAccounts().then(() => {
        this.selectedAcc = this.accounts[this.accounts.length - 1].name;
        this.changeSelect();
        this.step = "first";
      });
    },
    changeSelect() {
      const account = walletContext.wallets.find(
        (temp) => temp.name === this.selectedAcc
      );
      const list = [];
      this.validRegistrys.forEach((temp) => {
        try {
          temp.address = walletContext.getAddress(account, temp.prefix);
          list.push(temp);
        } catch (error) {
          console.log("exception===", temp);
        }
      });
      this.addressList = list;
      console.log("addressList", this.addressList);
    },
    showBalance(item) {
      console.log(item);
      walletContext.getBalance(item.network, item.address).then((val) => {
        item.balance = val;
      });
    },
    toTransfer(item) {
      console.log("transfer===", item);
      walletContext.transfer(item.network, item.address, item.toaddress);
    },
  },
};
</script>
<!-- // 5HRgRTeoPUHUxtaDLz31GcZiGwXPvwEVwcMTBrx5A1fu8BPk -->
