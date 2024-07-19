import {
  Center, Stack, Box, Heading, Button, Input,
  InputGroup, InputLeftAddon,
  Flex, Text, FormControl, FormLabel, Tooltip
} from "@chakra-ui/react";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { web3Accounts, web3Enable, web3FromSource } from '@polkadot/extension-dapp';
import { useState } from "react";

function Poe() {

  const [formData, setFormData] = useState({
    yourName: "",
  });

  const handleInputChange = (field, value) => {
    setFormData({
      ...formData,
      [field]: value,
    });
  };

  const handleSubmit =
    async () => {
      console.log("Form Submitted:", formData);
      await web3Enable("Using Wallet Extension");
      const encoder = new TextEncoder();
      const uintArrayValue = encoder.encode(formData.yourName);

      const network = "ws://127.0.0.1:9944";
      const wsProvider = new WsProvider(network);
      const allAccounts = await web3Accounts();
      const account = allAccounts[0];
      const api = await ApiPromise.create({ provider: wsProvider });

      localStorage.setItem("Publickey", account.address);
      const addressaccount: any = localStorage.getItem("Publickey");

      const claimHash = api.createType('Hash', api.registry.hash(uintArrayValue).toHex());
      console.log('claimHash ' + claimHash);
      const tx = api.tx.templateModule.doSomething(claimHash);

      const injector = await web3FromSource(account.meta.source);
      const hash3 = await tx.signAndSend(addressaccount, { signer: injector.signer });
      console.log(`PoE - hashed data - ${hash3.toHex()}`);

      if (!account) {
        console.log('Please login your wallet');
        return;
      }

      setFormData({
        yourName: "",
      });
    };

  return (
    <><Box w="100%" p={4}>
      <Flex justifyContent="center">
        <Flex
          direction="column"
          alignItems="center"
          w={{ base: "90%", md: "70%", lg: "50%" }}>
          <Text as="h2" fontWeight="bold">
            PoE
          </Text>
          <FormControl isRequired>
            <FormLabel>Your Name</FormLabel>
            <Input
              placeholder="Your Name"
              onChange={
                (e) =>
                  handleInputChange("yourName",
                    e.target.value)} />
          </FormControl>
          <Button
            variant="solid"
            mt="5"
            colorScheme="green"
            size="md"
            onClick={handleSubmit}>
            Submit
          </Button>
        </Flex>
      </Flex>
    </Box>

    </>
  );
}

export { Poe };
