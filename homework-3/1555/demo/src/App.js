import { Container,Row,Col} from 'react-bootstrap';
import { useEffect, useState } from "react";

function App() {


  const self={

  }

  useEffect(() => {

  }, []);

  return (
    <Container>
      <Row>
        <Col xl={12}>
          <h4>Account</h4>
        </Col>
        <Col>
          functions here
        </Col>
        <Col xl={12}>
          <hr />
        </Col>

        <Col xl={12}>
          <h4>Transaction</h4>
        </Col>
        <Col>
          functions here
        </Col>
        <Col xl={12}>
          <hr />
        </Col>

        <Col xl={12}>
          <h4>Connection</h4>
        </Col>
        <Col>
          functions here
        </Col>
        <Col xl={12}>
          <hr />
        </Col>
      </Row>
    </Container>
  );
}

export default App;
