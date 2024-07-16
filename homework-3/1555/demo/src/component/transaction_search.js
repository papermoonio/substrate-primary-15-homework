import { Row, Col, Table } from 'react-bootstrap';
import { useEffect, useState } from "react";
import { FaFaucet } from "react-icons/fa";

function TransactionSearch(props) {
  const size = {
    row: [12],
    search: [5, 2],
  };

  let [info, setInfo] = useState("");

  useEffect(() => {

  }, []);

  return (
    <Row className='pt-2'>
      <Col md={size.search[0]} lg={size.search[0]} xl={size.search[0]} xxl={size.search[0]}>
        <input type="text" className='form-control' placeholder='The transaction hash to search.' />
      </Col>
      <Col md={size.search[1]} lg={size.search[1]} xl={size.search[1]} xxl={size.search[1]}>
        <button className='btn btn-md btn-primary'>Search</button>
      </Col>
      <Col className='pt-2 text-end' md={size.row[0]} lg={size.row[0]} xl={size.row[0]} xxl={size.row[0]}>
        {info}
      </Col>
    </Row>
  );
}
export default TransactionSearch;