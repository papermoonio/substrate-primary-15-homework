import { Row, Col } from 'react-bootstrap';
import { useEffect, useState } from "react";

import { Keyring } from "@polkadot/api";

function PaymentDetails(props) {
  const size = {
    row: [12],
    half: [6, 6],
    divide:[3],
    transfer:[10,2],
  };

  let [info, setInfo]= useState("");

  let [from, setFrom]=useState("");
  let [balance, setBalance] =useState(0);
  let [free, setFree]=useState(0);
  let [amount, setAmount]=useState(0);
  let [target, setTarget]=useState("");
  let [password, setPassword]=useState("");

  const self={
    changeAccount:(ev)=>{
      const val=ev.target.value;
      setFrom(val);
      self.balance(val,props.API,(dt)=>{
        setBalance(dt.free*self.getDivide());
      });
    },
    changeAmount:(ev)=>{
      setAmount(parseInt(ev.target.value));
    },
    changeTarget:(ev)=>{
      setTarget(ev.target.value);
      self.balance(ev.target.value,props.API,(dt)=>{
        setFree(dt.free*self.getDivide());
      });
    },
    changePassword:(ev)=>{
      setPassword(ev.target.value);
    },

    clickTransfer:(ev)=>{
      setInfo("");
      const acc=self.getEncryJSON(from,props.list);
      const keyring = new Keyring({ type: 'sr25519' });
      try {
          const pair = keyring.createFromJson(acc);
          try {
              pair.decodePkcs8(password);
              const API = props.API;
              const m = self.getMulti();
              try {
                API.tx.balances.transferAllowDeath(target, parseInt(amount * m)).signAndSend(pair, (res) => {
                  const status = res.status.toJSON();
                  setInfo(JSON.stringify(status));
                  if(status.finalized){
                    setPassword("");
                    setAmount(0);

                    self.balance(from,API,(dt)=>{
                      setBalance(dt.free*self.getDivide());
                    });

                    self.balance(target,API,(dt)=>{
                      setFree(dt.free*self.getDivide());
                    });
                  }
                });
              } catch (error) {
                setInfo("Internal error.");
              }
          } catch (error) {
            setInfo("Invalid password.");
          }
      } catch (error) {
        setInfo("Invalid account JSON file.");
      }
    },
    getEncryJSON:(address,list)=>{
      for(let i=0;i<list.length;i++){
        if(list[i].address===address) return list[i];
      }
      return false;
    },
    getMulti: () => {
      return 1000000000000;
    },
    getDivide:()=>{
      return 0.000000000001;
    },
    
    balance: (address, API, ck) => {
      let unsub = null;
      API.query.system.account(address, (res) => {
        if (unsub != null) unsub();
        const data = res.toJSON().data;
        data.address = address;
        return ck && ck(data);
      }).then((fun) => {
        unsub = fun;
      });
    },
    fresh:()=>{
      //console.log(props.list);
      if(!props.API) return false;
      if(!props.list || props.list===null || props.list.length===0) return false;
      const account=props.list[0];

      setFrom(account.address);
      self.balance(account.address,props.API,(dt)=>{
        setBalance(dt.free*self.getDivide());
      });
    },
  }

  useEffect(() => {
    self.fresh();

  }, [props.API,props.list]);

  return (
    <Row className='pt-2'>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Pay from ( balance: {balance} )</small>
        <select className='form-control' onChange={(ev)=>{
          self.changeAccount(ev);
        }}>
          {props.list && props.list.map((row, index) => (
            <option key={index} value={row.address}>{row.address}</option>
          ))}
        </select>
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Pay to  ( balance: {free} )</small>
        <input type="text" className='form-control' placeholder='The address accept the coins.' value={target} onChange={(ev)=>{
          self.changeTarget(ev);
        }}/>
      </Col>
      <Col md={size.divide[0]} lg={size.divide[0]} xl={size.divide[0]} xxl={size.divide[0]}>
        <small>Password</small>
        <input type="password" className='form-control' placeholder='The password of account.'  value={password} onChange={(ev)=>{
          self.changePassword(ev);
        }}/>
      </Col>
      <Col md={size.divide[0]} lg={size.divide[0]} xl={size.divide[0]} xxl={size.divide[0]}>
      </Col>
      <Col md={size.divide[0]} lg={size.divide[0]} xl={size.divide[0]} xxl={size.divide[0]}>
        <small>Amount</small>
        <input type="number" className='form-control' placeholder='The amount of coins.' value={amount} onChange={(ev)=>{
          self.changeAmount(ev);
        }}/>
      </Col>
      <Col md={size.divide[0]} lg={size.divide[0]} xl={size.divide[0]} xxl={size.divide[0]}>
      </Col>
      <Col className='pt-2 text-end' md={size.transfer[0]} lg={size.transfer[0]} xl={size.transfer[0]} xxl={size.transfer[0]}>
          {info}
      </Col>
      <Col className='pt-2 text-end' md={size.transfer[1]} lg={size.transfer[1]} xl={size.transfer[1]} xxl={size.transfer[1]}>
          <button className='btn btn-md btn-primary' onClick={(ev)=>{
            self.clickTransfer(ev);
          }}>Transaction</button>
      </Col>
    </Row>
  );
}
export default PaymentDetails;