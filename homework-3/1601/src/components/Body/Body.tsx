import { Breadcrumb, BreadcrumbItem, BreadcrumbLink } from "@chakra-ui/react";
import { Link, BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { Portfolio, Accounts, Addaccounts, Newaccounts, Poe } from "../../pages";

function Main() {
  return (
    <Router>
      <div>
        <Breadcrumb px={5}>
          <BreadcrumbItem>
            <BreadcrumbLink fontWeight="bold" as={Link} to="/">
              Accounts
            </BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbItem>
            <BreadcrumbLink fontWeight="bold" as={Link} to="/portfolio">
              Tokens
            </BreadcrumbLink>
          </BreadcrumbItem>
        </Breadcrumb>
        <Routes>
          <Route path="/" element={<Accounts />} />
          <Route path="/portfolio" element={<Portfolio />} />
          <Route path="/newaccount" element={<Newaccounts />} />
          <Route path="/addaccount" element={<Addaccounts />} />
          <Route path="/poe" element={<Poe />} />
        </Routes>
      </div>
    </Router>
  );
}

export { Main };
