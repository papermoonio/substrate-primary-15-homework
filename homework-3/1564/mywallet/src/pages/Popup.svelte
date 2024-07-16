<script>
    import { onMount } from "svelte";
    import { walletContext } from "../context";
    import registryJson from "@substrate/ss58-registry";

    console.log("Hello from the popup!");
    let loading = true;
    let accounts = [];
    let step = "first";
    let newwords = "";
    let wordslen = "12";
    let importwords = "";
    let password = "";
    let errmsg = "";
    let selectedAcc = "";
    let addressList = [];

    console.log("registryJson===", registryJson);
    //
    let validRegistrys = registryJson.filter((temp) => temp.symbols.length > 0);

    $: accountlen = accounts.length;

    onMount(() => loadAccounts());

    function loadAccounts() {
        return walletContext.loadData().then(() => {
            Object.keys(walletContext.wallets).forEach((key) => {
                accounts.push({ name: key, data: walletContext.wallets[key] });
            });
            loading = false;
            accountlen = accounts.length;
        });
    }

    function createWallet() {
        const words = walletContext.generateWallet(12);
        console.log("words===", words);
        newwords = words;
        step = "second_create";
    }

    function regenerate() {
        const words = walletContext.generateWallet(parseInt(wordslen));
        console.log("words===", words);
        newwords = words;
    }

    function importWallet() {
        step = "second_import";
    }

    function confirmCreate() {
        walletContext.importWallet(newwords);
        step = "third_setpass";
    }

    function confirmImport() {
        const len = importwords.split(" ").length;
        // @ts-ignore
        if (len !== 12 || len !== 15 || len !== 18 || len !== 21 || len !== 24) {
        }
        walletContext.importWallet(importwords);
        step = "third_setpass";
    }

    function confirmPass() {
        step = "four_success";
    }

    function toStart() {
      loadAccounts().then(()=>{
          selectedAcc = "Account1";
          changeSelect();
        })
    }

    function changeSelect() {
        const account = walletContext.wallets[selectedAcc];
        let list =[];
         validRegistrys.forEach((temp) => {
            try {
              temp.symbol = temp.symbols[0];
              temp.address = walletContext.getAddress(account, temp.prefix);
              list.push(temp);
            } catch (error) {
              console.log("exception===",temp);
            }
           
        });
        addressList = list;
        console.log('addressList',addressList);
    }
</script>

<div class="popup-wrap">
    {#if loading}
        <div class="setup-loading">loading...</div>
    {/if}

    {#if accountlen === 0}
        <div class="setup-steps">
            {#if step === "first"}
                <div class="setup-box1">
                    <h3>welcome to mywallet</h3>
                    <div class="btns">
                        <button on:click={createWallet}>create wallet </button>
                        <button on:click={importWallet}>import wallet</button>
                    </div>
                </div>
            {/if}

            {#if step === "second_create"}
                <div class="setup-box2">
                    <h3>create wallet</h3>
                    <div>
                        <select name="wordslen" id="wordslen" bind:value={wordslen} on:change={regenerate}>
                            <option value="12">12</option>
                            <option value="15">15</option>
                            <option value="18">18</option>
                            <option value="21">21</option>
                            <option value="24">24</option>
                        </select>
                    </div>
                    <div class="word-list">
                        {newwords}
                    </div>
                    <div class="btns">
                        <button on:click={confirmCreate}>confirm</button>
                    </div>
                </div>
            {/if}
            {#if step === "second_import"}
                <div class="setup-box2">
                    <h3>import wallet</h3>
                    <div class="word-list">
                        <input bind:value={importwords} />
                    </div>
                    <div class="errormsg">{errmsg}</div>
                    <div class="btns">
                        <button on:click={confirmImport}>confirm</button>
                    </div>
                </div>
            {/if}

            {#if step === "third_setpass"}
                <div class="setup-box3">
                    <h3>set password</h3>
                    <div class="input-pass">
                        <input bind:value={password} />
                    </div>
                    <div class="errormsg">{errmsg}</div>
                    <div class="btns">
                        <button on:click={confirmPass}>confirm</button>
                    </div>
                </div>
            {/if}
            {#if step === "four_success"}
                <div class="setup-box4">
                    <h3>result</h3>
                    <div class="input-pass">successed</div>
                    <div class="btns">
                        <button on:click={toStart}>start</button>
                    </div>
                </div>
            {/if}
        </div>
    {/if}

    {#if accountlen !== 0}
        <div>
            <h1>welcome to mywallet by xuxihai</h1>
            <!-- <div class="password-verify">
                <input type="password" placeholder="please input password" />
                <button>unlock</button>
            </div> -->

            <div class="accounts-list">
                <select bind:value={selectedAcc} on:change={changeSelect}>
                    {#each accounts as item}
                        <option value={item.name}>
                            {item.name}
                        </option>
                    {/each}
                </select>
            </div>
            <div class="address-lsit">
                <ul>
                    {#each addressList as item, i}
                        <li>
                            {i + 1}: {item.address} - {item.symbol}- {item.network}- {item.balance}
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
    {/if}
</div>

<!-- 1. 可以新建帐号，查看余额
2. 显示钱包地址
3. 转账到其他帐号
4. 入账提醒
5. 调用特定Substrate链的交易 （PoE链）
6. 参考知名钱包的任何功能 （可选） -->

<style lang="scss">
    .popup-wrap {
        background-color: #fff;
        width: 660px;
        height: 780px;
        overflow-y: scroll;
        h3 {
            line-height: 36px;
            height: 36px;
            padding: 0 12px;
        }
    }
</style>
