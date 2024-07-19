import { useToast, Heading, Flex, Spacer, Stack } from "@chakra-ui/react";
import { useState, useEffect } from "react";
import { ApiPromise, WsProvider } from "@polkadot/api";

function Balance({ wsEndpoint, token }) {

  const publicKey = localStorage.getItem("Publickey");

  const [account, setAccount] = useState(publicKey);
  const [Balances, setBalances] = useState(0);

  const toast = useToast();

  const wsProvider = new WsProvider(wsEndpoint);
  const Balance = async function (account: any) {
    const api = await ApiPromise.create({ provider: wsProvider });
    setAccount(localStorage.getItem("Publickey"));
    let {
      data: { free: previousFree },
      nonce: previousNonce,
    }: any = await api.query.system.account(account);
    const amount = Number(previousFree) / 1000000000000;
    setBalances(amount);

    api.query.system.account(
      account,
      ({ data: { free: currentFree }, nonce: currentNonce }: any) => {
        const change = currentFree.sub(previousFree);

        if (!change.isZero()) {
          previousFree = currentFree;
          previousNonce = currentNonce;
          const Newamount = Number(previousFree) / 1000000000000;
          setBalances(Newamount);
        }
      }
    );

    api.query.system.events((events:any) => {
      events.forEach((record:any) => {
        const { event } = record;
        if (api.events.balances.Transfer.is(event)) {
          const [from, to, amount]:any = event.data;
          if (to.toString() === account) {
            toast({
              title: "Received Fund is" + (amount/1000000000000),
              duration: 6000,
              status: "success",
              isClosable: true,
            });
            console.log(`Incoming transfer: ${amount} units from ${from}`);
          }
        }
      });
    });
  };

  useEffect(() => {
    Balance(account);
  });

  return (
    <Stack alignItems="center">
      <Heading size="md"> Balance</Heading>
      <Flex gap="30px">
        <Heading size="md">{token}</Heading>
        <Spacer />
        <Heading size="md">{Balances.toFixed(4)}</Heading>
      </Flex>
    </Stack>
  );
}

export { Balance };
