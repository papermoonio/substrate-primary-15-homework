import {
  Button,
  ButtonGroup,
  Stack,
  Text,
  Divider,
  Heading,
  Image,
  Card,
  CardHeader,
  CardBody,
  CardFooter,
} from "@chakra-ui/react";
import { Link } from "react-router-dom";
import { BiAddToQueue } from "react-icons/bi";
import "./app.css";

function Addaccount() {
  return (
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
          <Stack alignItems="center">
            <Heading color="pink.600" size="sm">
              {" "}
              Add account
            </Heading>
          </Stack>
        </CardBody>
        <Divider />
        <CardFooter>
          <Button as={Link} to="/addaccount" variant="ghost" colorScheme="pink">
            <BiAddToQueue color="pink.600" size="40px" />
          </Button>
        </CardFooter>
      </Stack>
    </Card>
  );
}

export { Addaccount };
