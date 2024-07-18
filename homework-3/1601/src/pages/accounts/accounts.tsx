import { Center, Flex, Stack, Box, Heading } from "@chakra-ui/react";
import { Addaccount } from "./Addaccount";
import { Newaccount } from "./Newaccount";
import { Poe } from "../poe";

function Accounts() {
  return (
    <>
      <Center fontWeight="bold">
        <Heading size="md">Accounts</Heading>
      </Center>
      <Stack alignItems="center" h="400px" w="100%">
        <Flex alignItems="center" height="100vh" gap="10">
          <Addaccount />
          <Newaccount />
        </Flex>
      </Stack>
      <Poe />
      <Box h="120px" w="100%" />
    </>
  );
}

export { Accounts };
