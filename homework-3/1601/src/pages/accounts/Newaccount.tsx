import {
  Button,
  Stack,
  Divider,
  Heading,
  Card,
  CardBody,
  CardFooter,
} from "@chakra-ui/react";
import { Link } from "react-router-dom";
import { MdOutlineGroupAdd } from "react-icons/md";
import "./app.css";

function Newaccount() {
  return (
    <>
      <Card
        className="increaseshadow"
        w="200px"
        h="200px"
        borderColor="gray.400"
        borderRadius="md"
        boxShadow="dark-lg"
        p="4"
        position="relative"
      >
        <Stack alignItems="center">
          <CardBody>
            <Heading color="pink.600" size="sm">
              {" "}
              New account
            </Heading>
          </CardBody>
          <Divider />
          <CardFooter>
            <Button
              as={Link}
              to="/newaccount"
              variant="ghost"
              colorScheme="pink"
            >
              <MdOutlineGroupAdd color="pink.600" size="40px" />
            </Button>
          </CardFooter>
        </Stack>
      </Card>
    </>
  );
}

export { Newaccount };
