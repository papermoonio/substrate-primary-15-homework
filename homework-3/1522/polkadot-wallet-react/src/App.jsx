import { useState } from 'react'
import './App.css'
import Wallet from './Wallet';
function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <h1>Polkadot Wallet</h1>
      <Wallet />
    </div>
  )
}

export default App
