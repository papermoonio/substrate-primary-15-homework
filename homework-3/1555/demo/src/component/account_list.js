import { Row, Col, Table, Badge } from 'react-bootstrap';
import { useEffect, useState } from "react";
import { FaFileDownload, FaFaucet } from "react-icons/fa";

import { Keyring } from "@polkadot/api";

function AccountList(props) {
  const size = {
    row: [12],
  };

  let [info, setInfo] = useState("");
  let [map, setMap] = useState({});

  const self = {
    clickFaucet: (address) => {
      const amount = self.rand(200, 1000);
      setInfo(`Start to transfer ${amount} from Alice to ${address}`);
      setTimeout(()=>{
        self.faucet(address, amount, (res) => {
          if(res.error) return setInfo(res.error);
          self.fresh();
        });
      },1500);
    },
    clickDownload: (index) => {
      const acc=props.list[index];
      const dt=JSON.parse(JSON.stringify(acc));
      delete dt.password;
      delete dt.mnemonic;
      dt.meta={from:"demo",auth:"Fuu"}
      self.download(`${acc.address}.json`,JSON.stringify(dt));
    },
    getMulti: () => {
      return 1000000000000;
    },
    getDivide:()=>{
      return 0.000000000001;
    },
    getHumanAmount:(free)=>{
      return (free*self.getDivide()).toLocaleString();
    },
    faucet: (address, amount, ck) => {
      setInfo("");
      const keyring = new Keyring({ type: 'sr25519' });
      const pair = keyring.addFromUri('//Alice');
      const API = props.API;
      const m = self.getMulti();
      try {
        API.tx.balances.transferAllowDeath(address, parseInt(amount * m)).signAndSend(pair, (res) => {
          const status = res.status.toJSON();
          setInfo(JSON.stringify(status));
          if(status.finalized) return ck && ck(status.finalized);
        });
      } catch (error) {
        return ck && ck({ error: "Internal error." });
      }
    },
    rand: (m, n) => {
      return Math.round(Math.random() * (m - n) + n);
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
      setInfo("");
      if (props.API === null) return setInfo("No websocket link yet.");
      if (props.list.length === 0) return true;
      let work = 0;
      for (let i = 0; i < props.list.length; i++) {
        work++;
        const row = props.list[i];
        self.balance(row.address, props.API, (data) => {
          map[data.address] = data;
          work--;
          if (work < 1) {
            //console.log(map);
            setMap(JSON.parse(JSON.stringify(map)));
          }
        });
      }
    },
    download: (filename, text, type) => {
      const element = document.createElement('a');
      switch (type) {
        case "image":
          element.setAttribute('href', text);
          break;
  
        default:
          element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
          break;
      }
  
      element.setAttribute('download', filename);
  
      element.style.display = 'none';
      document.body.appendChild(element);
  
      element.click();
      document.body.removeChild(element);
    },
  }

  useEffect(() => {
    self.fresh();
  }, [props.API, props.list]);

  return (
    <Row className='pt-4'>
      <Col className='pt-2' md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        <Badge>Important</Badge> faucet units is from default account <strong>Alice</strong> of substrate dev node.
      </Col>
      <Col className='pt-2' md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        <Table striped bordered hover>
          <thead>
            <tr>
              <th>Address</th>
              <th>Balance</th>
              <th>Password</th>
              <th>Mnemonic</th>
              <th>Download</th>
              <th>Faucet</th>
            </tr>
          </thead>
          <tbody>
            {props.list && props.list.map((row, index) => (
              <tr key={index}>
                <td>{row.address}</td>
                <td>{!map[row.address] ? 0 : self.getHumanAmount(map[row.address].free)}</td>
                <td>{row.password}</td>
                <td>{row.mnemonic}</td>
                <td className='text-center'>
                  <button className='btn btn-md btn-default text-center text-primary' onClick={(ev) => {
                    self.clickDownload(index);
                  }}><FaFileDownload size={20} /></button>
                </td>
                <td className='text-center'>
                  <button className='btn btn-md btn-default text-center text-primary' onClick={(ev) => {
                    self.clickFaucet(row.address);
                  }}><FaFaucet size={20} /></button>
                </td>
              </tr>
            ))}
          </tbody>
        </Table>
      </Col>
      <Col className='text-end' md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        {info}
      </Col>

    </Row>
  );
}
export default AccountList;