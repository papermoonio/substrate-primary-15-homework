import { useState } from "react";
import {
  Heading,
  Box,
  Button,
  HStack,
  Spacer,
  Image,
} from "@chakra-ui/react";
import Identicon from "@polkadot/react-identicon";
import { AccountButton } from "./AccountModal";
import "./app.css";

function Header() {
  const size = 33;
  const theme = "polkadot";
  const publicKey = localStorage.getItem("Publickey");
  const [Account] = useState(publicKey);

  return (
    <>
      <Box w="100%" h="20px" />
      <HStack>
        <Box p={5} >
          <HStack>
            <Image
              boxSize="40px"
              objectFit="cover"
              alignItems="center"
              src={
                "https://polkadot.js.org/logo.svg"
              }
              borderRadius="lg"
            />
            <Heading size="sm">polkadotjs wallet</Heading>
          </HStack>
        </Box>
        <Spacer />
        <Box></Box>
        <Box p={5} >
          <Button backgroundColor="orange.400" borderRadius="25px">
            <Identicon value={Account} size={size} theme={theme} />
            <AccountButton/> 
          </Button>
        </Box>
      </HStack>
      <Box w="100%" h="25px" />
    </>
  );
}

export { Header };