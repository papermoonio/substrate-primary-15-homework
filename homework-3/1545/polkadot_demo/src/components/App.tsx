import { useState } from 'react'
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api'
import { Layout, Flex, Input, Button, message } from 'antd'

const RPC_URL = 'ws://127.0.0.1:9944'

function App() {
    const [api, setApi] = useState(null)
    const [messageApi, contextHolder] = message.useMessage()

    const [account, setAccount] = useState(null)
    const [accountMne, setAccountMne] = useState('')
    const [accountAddr, setAccountAddr] = useState('')
    const [accountBal, setAccountBal] = useState('')

    const [toAddr, setToAddr] = useState('')
    const [amount, setAmount] = useState('')

    const [queryBalAddr, setQueryBalAddr] = useState('')
    const [queryBal, setQueryBal] = useState('')

    const [poeData, setPoeData] = useState('')

    const connect = async () => {
        const provider = new WsProvider(RPC_URL)
        if (api == null) {
            const _api = await ApiPromise.create({ provider, types: {} })
            setApi(_api)
            return _api
        } else {
            return api
        }
    }

    const handleNew = async () => {
        const _api = await connect()

        if (!accountMne.trim()) {
            alert('Please input account mnemonic')
            return
        }

        const keyring = new Keyring({ type: 'sr25519' })
        const account = keyring.addFromUri(accountMne)
        setAccount(account)
        setAccountAddr(account.address)
        console.log(account)

        const { data : {free: bal}} = await _api.query.system.account(account.address)
        console.log(bal.toString())
        setAccountBal(bal.toString())

        // subscribe deposit
        await _api.query.system.events((events) => {
            events.forEach(({ event }) => {
                if (event.section === 'balances' && event.method === 'Transfer') {
                    const [from, to, amount] = event.data
                    if (to == account.address) {
                        console.log(`Received ${amount} units from ${from}`)
                        messageApi.success(`Received ${amount} units from ${from}`)
                    }
                }
            })
        })
    }

    const handleQueryBal = async () => {
        const _api = await connect()

        if (!queryBalAddr.trim()) {
            alert('Please input address')
            return
        }

        const { data : {free: bal}} = await _api.query.system.account(queryBalAddr)
        console.log(bal.toString())
        setQueryBal(bal.toString())
    }

    const handleTransfer = async () => {
        const _api = await connect()

        if (!accountMne.trim()) {
            alert('Please load account')
            return
        }

        if (!toAddr.trim() || !amount.trim()) {
            alert('Please input toAddr or amount')
            return
        }

        const tx = _api.tx.balances.transferKeepAlive(toAddr, Number(amount))
        const hash = await tx.signAndSend(account)
        console.log(hash.toHex())
    }

    const handlePoe = async () => {
        const _api = await connect()

        if (!poeData.trim()) {
            alert('Please load account')
            return
        }

	if (isNaN(Number(poeData))) {
            alert('Please input number')
            return
        }

        const tx = _api.tx.poeModule.createClaim(Number(poeData))
        const hash = await tx.signAndSend(account)
        console.log(hash.toHex())
        messageApi.success(`PoE hash ${hash}`)
    }

    return (
      <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'left', gap: '10px' }}>
        {contextHolder}
        <div>
          <Button
              style={{ width: '200px', textAlign: 'center', marginRight: '10px' }}
              type="primary"
              size="large"
              onClick={handleNew}>New Account
          </Button>
          <Input
              style={{ width: '300px' }}
              size="large"
              type="text"
              value={accountMne}
              onChange={(e) => setAccountMne(e.target.value)}
              placeholder="Mnemonic"
          />
          {accountAddr && (
              <div>
                  <p>mnemonic: {accountMne}</p>
                  <p>address: {accountAddr}</p>
                  <p>balance: {accountBal}</p>
              </div>
          )}
        </div>
        <div>
          <Button
              style={{ width: '200px', textAlign: 'center', marginRight: '10px' }}
              type="primary"
              size="large"
              onClick={handleTransfer}>Transfer
          </Button>
          <Input
              style={{ width: '300px', marginRight: '10px' }}
              size="large"
              type="text"
              value={toAddr}
              onChange={(e) => setToAddr(e.target.value)}
              placeholder="To Address"
          />
          <Input
              style={{ width: '100px' }}
              size="large"
              type="text"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="Amount"
          />
        </div>
        <div>
          <Button
              style={{ width: '200px', textAlign: 'center', marginRight: '10px' }}
              type="primary"
              size="large"
              onClick={handleQueryBal}>Balance
          </Button>
          <Input
              style={{ width: '300px' }}
              size="large"
              type="text"
              value={queryBalAddr}
              onChange={(e) => setQueryBalAddr(e.target.value)}
              placeholder="Address"
          />
          {queryBal && (
              <div>
                  <p>address: {queryBalAddr}</p>
                  <p>balance: {queryBal}</p>
              </div>
          )}
        </div>
        <div>
          <Button
              style={{ width: '200px', textAlign: 'center', marginRight: '10px' }}
              type="primary"
              size="large"
              onClick={handlePoe}>PoE
          </Button>
          <Input
              style={{ width: '300px' }}
              size="large"
              type="text"
              value={poeData}
              onChange={(e) => setPoeData(e.target.value)}
              placeholder="Data"
          />
        </div>
      </div>
    )
}

export default App;

