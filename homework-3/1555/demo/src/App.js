import { Container, Tabs, Tab, Row, Col, Toast } from 'react-bootstrap';
import { useEffect, useState } from "react";

import { ApiPromise, WsProvider } from "@polkadot/api";

import AccountAdd from './component/account_add';
import AccountList from './component/account_list';
import PaymentDetails from './component/payment_details';
import AcnhorSet from './component/anchor_set';
import TransactionSearch from './component/transaction_search';

let wsAPI = null;
let linking = false;

function App() {
  const size = {
    row: [12],
    network:[3,4,4],
  }
  const nodes = [
    "ws://localhost:9944",
    "wss://dev2.metanchor.net",
  ];

  let [accounts, setAccounts] = useState([]);
  let [info, setInfo] = useState("");
  let [eventInfo,setEventInfo]= useState("");

  const self = {
    changeNetwork:(ev)=>{
      wsAPI=null;
      linking=false;
      const num=parseInt(ev.target.value);
      self.fresh(num);
    },
    callbackNewAccount: (acc) => {
      const nlist=[acc];
      for(let i=0;i<accounts.length;i++){
        nlist.push(accounts[i]);
      }
      setAccounts(nlist);
    },
    init: (ck,node) => {
      const uri = nodes[node];
      if (linking) return setTimeout(() => {
        self.init(ck);
      }, 500);

      if (wsAPI !== null) return ck && ck(wsAPI);

      linking = true;
      const provider = new WsProvider(uri);
      ApiPromise.create({ provider: provider }).then((api) => {
        console.log(`Linked to node ${uri}`);
        wsAPI = api;
        linking = false;
        return ck && ck(wsAPI);
      }).catch((error) => {
        linking = false;
        return ck && ck(error);
      });
    },
    decode:(list)=>{
      for(let i=0;i<list.length;i++){
        const row=list[i];
        //console.log(row);
        if(row.event && 
            row.event.data &&
            row.event.index==="0x0602" 
        ){
          console.log(row);
          const data=row.event.data;
          setEventInfo(`${data[1]} get ${data[2]*0.000000000001} coins from ${data[0]}.`);
          setTimeout(()=>{
            setEventInfo("");
          },8000);
        }
      }
    },
    fresh:(n)=>{
      //console.log(n);
      setInfo(`Ready to link to node: ${nodes[n]}`);
      self.init((wsAPI) => {
        setInfo(`Linked to node: ${nodes[n]}`);
        // wsAPI.events.balances.Transfer.is((from,to,amount)=>{
        //   console.log(from,to,amount);
        // });
        console.log(wsAPI.query.system.events);
        wsAPI.query.system.events((evs)=>{
          const arr=evs.toJSON();
          //console.log(arr);
          self.decode(arr);
        });
      },n);
    },
  }

  useEffect(() => {
    self.fresh(0);
  }, []);

  return (
    <Container>
      <Row className='pt-2 pb-4'>
        <Col md={size.network[0]} lg={size.network[0]} xl={size.network[0]} xxl={size.network[0]}>
          <select className='form-control' onChange={(ev)=>{
            self.changeNetwork(ev);
          }}>
            {nodes.map((row, index) => (
              <option key={index} value={index}>{row}</option>
            ))}
          </select>
        </Col>
        <Col className='pt-2' md={size.network[1]} lg={size.network[1]} xl={size.network[1]} xxl={size.network[1]}>
          {info}
        </Col>
        
      </Row>

      <Row>
        <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
          <h3>Functions</h3>
        </Col>
    </Row>
      <Tabs
        defaultActiveKey="account"
        id="uncontrolled-tab-example"
        className="mb-3"
      >
        <Tab eventKey="account" title="Account Management">
        
          <AccountAdd API={wsAPI} callback={(json,mnemonic) => {
            self.callbackNewAccount(json);
          }} />
          <Row>
            <Col className='pt-2 text-warning' md={size.row[2]} lg={size.row[2]} xl={size.row[2]} xxl={size.row[2]}>
              {eventInfo}
            </Col>
          </Row>
          <AccountList API={wsAPI} list={accounts}/>
        </Tab>
        <Tab eventKey="payment" title="Payment Details">
          <PaymentDetails   API={wsAPI}  list={accounts} />
        </Tab>
        {/* <Tab eventKey="transaction" title="Transaction Search">
          <TransactionSearch  API={wsAPI} />
        </Tab> */}
        <Tab eventKey="anchor" title="Set Anchor">
          <AcnhorSet API={wsAPI}  list={accounts} />
        </Tab>
      </Tabs>
      {/* <Toast show={true} onClose={()=>{}}>
          <Toast.Header>
            <strong className="me-auto">Bootstrap</strong>
            <small>11 mins ago</small>
          </Toast.Header>
          <Toast.Body>Woohoo, you're reading this text in a Toast!</Toast.Body>
        </Toast> */}
    </Container>
    
  );
}

export default App;
