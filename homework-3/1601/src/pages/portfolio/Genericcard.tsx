import {
  Stack,
  Divider,
  Image,
  Card,
  CardBody,
  CardFooter,
} from "@chakra-ui/react";
import { Transfer } from "./Transfer";
import { RecieveModal } from "./Recieve";
import { Balance } from "./Balance";
import "./app.css";

function GenericCard({ image, network, token }) {
  return (
    <Card
      alignItems="center"
      className="increase"
      w="300px"
      h="350px"
      borderColor="gray.400"
      borderRadius="md"
      boxShadow="dark-lg"
    >
      <CardBody>
        <Stack alignItems="center" mt="6" spacing="3">
          <Image
            boxSize="80px"
            objectFit="cover"
            alignItems="center"
            src={image}
            borderRadius="lg"
          />
          <Balance wsEndpoint={network} token={token} />
        </Stack>
      </CardBody>
      <Divider />
      <CardFooter>
        <Stack direction="row" alignItems="center" spacing="10">
          <Transfer wsEndpoint={network} token={token} />
          <RecieveModal />
        </Stack>
      </CardFooter>
    </Card>
  );
}

export { GenericCard };
