import { Row, Col, Table, Badge } from 'react-bootstrap';
import { useEffect, useState } from "react";
import { FaFileDownload, FaFaucet } from "react-icons/fa";

function AccountList(props) {
  const size = {
    row: [12],
  };

  let [info, setInfo]=useState("");

  const self={
    clickFaucet:(address)=>{

    },
    clickDownload:(index)=>{

    },
  }

  useEffect(() => {

  }, []);

  return (
    <Row className='pt-4'>
      <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        {info}
      </Col>
      <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
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
                <td></td>
                <td>{row.password}</td>
                <td>{row.mnemonic}</td>
                <td>
                  <button className='btn btn-md btn-default text-center text-primary' onClick={(ev)=>{
                    self.clickDownload(index);
                  }}><FaFileDownload size={24} /></button></td>
                <td><button className='btn btn-md btn-default text-center text-primary' onClick={(ev)=>{
                    self.clickFaucet(row.address);
                  }}><FaFaucet size={24} /></button></td>
              </tr>
            ))}
          </tbody>
        </Table>
      </Col>
      <Col md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        <Badge>Important</Badge> faucet units is from default account Alice of substrate dev node.
      </Col>
    </Row>
  );
}
export default AccountList;