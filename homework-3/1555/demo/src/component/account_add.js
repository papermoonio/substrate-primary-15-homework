import { Row, Col } from 'react-bootstrap';
import { useEffect, useState } from "react";

import { Keyring } from "@polkadot/api";
import { mnemonicGenerate } from "@polkadot/util-crypto";

function AccountAdd(props) {
  const size = {
    row: [12],
    add: [4, 3]
  };

  let [disable, setDisable] = useState(true);
  let [password, setPassword] = useState("");
  let [info, setInfo] = useState("");
  let [list, setList] = useState([]);

  const self = {
    clickAdd: () => {
      const mnemonic = mnemonicGenerate();
      const keyring = new Keyring({ type: "sr25519" });
      const pair = keyring.addFromUri(mnemonic);
      const sign = pair.toJson(password);
      sign.password=password;       //!important, just for demo, please don't save password anywhere
      sign.mnemonic=mnemonic;       //!important, just for demo, please don't save mnemonic anywhere
      if(props.callback) props.callback(sign);
    },
    changePassword: (ev) => {
      setPassword(ev.target.value);
    },
  }

  useEffect(() => {
    setDisable(props.API !== null ? false : true);
  }, [props.API]);

  return (
    <Row className=''>
      <Col md={size.add[0]} lg={size.add[0]} xl={size.add[0]} xxl={size.add[0]}>
        <input type="text" className='form-control' value={password} placeholder='The password for new account' onChange={(ev) => {
          self.changePassword(ev);
        }} />
      </Col>
      <Col className='text-end' md={size.add[1]} lg={size.add[1]} xl={size.add[1]} xxl={size.add[1]}>
        <button disabled={disable} className='btn btn-md btn-primary' onClick={(ev) => {
          self.clickAdd(ev);
        }}>Generate New Account</button>
      </Col>
      <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        {info}
      </Col>
    </Row>
  );
}
export default AccountAdd;