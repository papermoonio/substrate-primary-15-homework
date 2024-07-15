import { Row, Col, Table } from 'react-bootstrap';
import { useEffect, useState } from "react";
import { FaFaucet } from "react-icons/fa";

function PaymentDetails(props) {
  const size = {
    row: [12],
    half: [6, 6],
  };

  let [info, setInfo]= useState("");

  useEffect(() => {

  }, []);

  return (
    <Row className='pt-2'>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Pay from</small>
        <select className='form-control'>
          {props.list && props.list.map((row, index) => (
            <option key={index} value={row.address}>{row.address}</option>
          ))}
        </select>
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Pay to</small>
        <input type="text" className='form-control' placeholder='The address accept the coins.' />
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>
        <small>Amount</small>
        <input type="number" className='form-control' placeholder='The amount of coins.' />
      </Col>
      <Col md={size.half[0]} lg={size.half[0]} xl={size.half[0]} xxl={size.half[0]}>{info}</Col>
      <Col className='pt-2 text-end' md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
          <button className='btn btn-md btn-primary'>Transaction</button>
      </Col>
    </Row>
  );
}
export default PaymentDetails;