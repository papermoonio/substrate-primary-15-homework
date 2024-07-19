import { Row, Col } from 'react-bootstrap';
import { useEffect, useState } from "react";

import { Keyring } from "@polkadot/api";

function AcnhorSet(props) {
  const size = {
    row: [12],
    half: [6, 6],
    divide: [3],
    transfer: [10, 2],
  };

  let [disable, setDisable] = useState(true);
  let [writing,setWriting] = useState(false);
  let [info, setInfo] = useState("");
  let [anchorInfo,setAnchorInfo]= useState("");

  let [from, setFrom] = useState("");
  let [balance, setBalance] = useState(0);
  let [password, setPassword] = useState("");

  let [name, setName] = useState("");
  let [raw, setRaw] = useState(JSON.stringify({ hello: "world" }));
  let [protocol, setProtocol] = useState(JSON.stringify({ fmt: "data" }));
  let [pre, setPre] = useState(0);

  const Anchor = {
    view: (name, ck) => {
      const API = props.API;
      if (!API) return false;
      let unsub = null;
      API.query.anchor.anchorOwner(name, (res) => {
        unsub();
        const dt = res.toJSON();
        if (!dt) return ck && ck(false);
        return ck && ck({ address: dt[0], block: dt[1] });
      }).then((fun) => {
        unsub = fun;
      });
    },
    write: (pair, name, raw, protocol, pre,ck) => {
      const API = props.API;
      if (!API) return false;
      try {
        API.tx.anchor.setAnchor(name, raw, protocol, pre).signAndSend(pair, (res) => {
            //const dt = res.toHuman();
            const status = res.status.toJSON();
            ck && ck(status);
        });
    } catch (error) {
        return ck && ck(error);
    }
    },
  }

  const self = {
    changeAccount: (ev) => {
      const val = ev.target.value;
      setFrom(val);
      self.balance(val, props.API, (dt) => {
        setBalance(dt.free * self.getDivide());
      });
    },
    changePassword: (ev) => {
      setPassword(ev.target.value);
    },

    changeName: (ev) => {
      setAnchorInfo("");
      setDisable(false);
      const anchor = ev.target.value
      setName(anchor);
      Anchor.view(anchor, (res) => {
        if(res!==false){
          setAnchorInfo(JSON.stringify(res));
          setDisable(true);
        }
      });
    },
    changeRaw: (ev) => {
      setRaw(ev.target.value);
    },
    changeProtocol: (ev) => {
      setProtocol(ev.target.value);
    },
    clickSetAnchor: (ev) => {
      setInfo("");
      if(!name) return setInfo("Anchor name required.");
      if(!raw) return setInfo("Anchor raw data required.");
      if(!protocol) return setInfo("Anchor protocol data required.");
      
      setWriting(true);

      const acc=self.getEncryJSON(from,props.list);
      const keyring = new Keyring({ type: 'sr25519' });
      try {
          const pair = keyring.createFromJson(acc);
          setPassword("");
          try {
              pair.decodePkcs8(password);
              Anchor.write(pair,name,raw,protocol,pre,(status)=>{
                setName("");
                setInfo(JSON.stringify(status));
                if(status.finalized){
                  setWriting(false);
                }
              });
          } catch (error) {
            setInfo("Invalid password.");
          }
      } catch (error) {
        setInfo("Invalid account JSON file.");
      }
    },
    getEncryJSON: (address, list) => {
      for (let i = 0; i < list.length; i++) {
        if (list[i].address === address) return list[i];
      }
      return false;
    },
    getMulti: () => {
      return 1000000000000;
    },
    getDivide: () => {
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
    fresh: () => {
      if (!props.API) return false;
      if (!props.list || props.list === null || props.list.length === 0) return false;
      const account = props.list[0];

      setFrom(account.address);
      self.balance(account.address, props.API, (dt) => {
        setBalance(dt.free * self.getDivide());
      });
    },
  }

  useEffect(() => {
    self.fresh();
    setDisable(props.API !== null ? false : true);
  }, [props.API, props.list]);

  return (
    <Row className='pt-2'>
      <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        Checking the pallet first.
      </Col>

      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Pay from ( balance: {balance} )</small>
        <select disabled={writing} className='form-control' onChange={(ev) => {
          self.changeAccount(ev);
        }}>
          {props.list && props.list.map((row, index) => (
            <option key={index} value={row.address}>{row.address}</option>
          ))}
        </select>
      </Col>
      <Col md={size.divide[0]} lg={size.divide[0]} xl={size.divide[0]} xxl={size.divide[0]}>
        <small>Password</small>
        <input disabled={writing}  type="password" className='form-control' placeholder='The password of account.' value={password} onChange={(ev) => {
          self.changePassword(ev);
        }} />
      </Col>
      <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        <hr />
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Name</small>
        <input disabled={writing}  type="text" className='form-control' placeholder='The name of anchor.' value={name} onChange={(ev) => {
          self.changeName(ev);
        }} />
      </Col>
      <Col className='pt-4' md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        {anchorInfo}
      </Col>

      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Raw Data</small>
        <textarea disabled={writing}  className='form-control' placeholder='Any data to storage on chain.' value={raw} onChange={(ev) => {
          self.changeRaw(ev);
        }}></textarea>
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
      </Col>

      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Protocol</small>
        <input disabled={writing}  type="text" className='form-control' placeholder='Protocol detail.' value={protocol} onChange={(ev) => {
          self.changeProtocol(ev);
        }} />
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
      </Col>

      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Previous Block Number</small>
        <input type="number" className='form-control' disabled placeholder='The previous block number.' value={pre} onChange={(ev) => {

        }} />
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
      </Col>

      <Col className='pt-2 text-end' md={size.transfer[0]} lg={size.transfer[0]} xl={size.transfer[0]} xxl={size.transfer[0]}>
        {info}
      </Col>
      <Col className='pt-2 text-end' md={size.transfer[1]} lg={size.transfer[1]} xl={size.transfer[1]} xxl={size.transfer[1]}>
        <button className='btn btn-md btn-primary' disabled={disable || writing} onClick={(ev) => {
          self.clickSetAnchor(ev);
        }}>Set Anchor</button>
      </Col>
    </Row>
  );
}
export default AcnhorSet;