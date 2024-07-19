import {
  Heading,
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  Center,
  Stack,
  VStack,
  Flex,
} from "@chakra-ui/react";
import { GenericCard } from "./Genericcard";

const image = "https://s2.coinmarketcap.com/static/img/coins/64x64/5034.png";
const network = "wss://rpc.ibp.network/westend";
const token = "KSM";

function Portfolio() {
  const allassets = [
    {
      image:
        "https://polkadot.study/img/polkadot_pink.svg",
      token: "DOT",
      //network: "wss://rococo-contracts-rpc.polkadot.io",
      network: "ws://127.0.0.1:9944",
    },
    {
      image: "https://cryptologos.cc/logos/kusama-ksm-logo.png",
      token: "KSM",
      network: "wss://westend-rpc.polkadot.io",
    },
    {
      image: "https://cryptologos.cc/logos/tether-usdt-logo.png?v=032",
      token: "USDT",
      network: "wss://1rpc.io/ksm",
    },
  ];

  const polkadotassets = [
    {
      image:
        "https://polkadot.study/img/polkadot_pink.svg",
      token: "DOT",
      network: "wss://rococo-contracts-rpc.polkadot.io",
    },
  ];

  const kusamaassets = [
    {
      image: "https://cryptologos.cc/logos/kusama-ksm-logo.png",
      token: "KSM",
      network: "wss://1rpc.io/ksm",
    },
  ];

  return (
    <>
      <Center fontWeight="bold">
        <Heading size="md">Tokens</Heading>
      </Center>
      <Tabs>
        <TabList>
          <Tab fontWeight="bold">All Tokens</Tab>
          <Tab fontWeight="bold">Polkadot</Tab>
          <Tab fontWeight="bold">Kusama</Tab>
        </TabList>

        <TabPanels>
          <TabPanel fontWeight="bold">
            <VStack spacing="10">
              <Heading size="md" alignItems="center">
                
              </Heading>
              <Stack direction="row" alignItems="center" spacing="10">
                <Flex direction="row" gap="30px">
                  {allassets.map((prop) => (
                    <GenericCard
                      image={prop.image}
                      network={prop.network}
                      token={prop.token}
                    />
                  ))}
                </Flex>
              </Stack>
            </VStack>
          </TabPanel>
          <TabPanel fontWeight="bold">
            <VStack spacing="10">
              <Heading size="md">Polkadot</Heading>
              <Flex direction="row" gap="30px">
                {polkadotassets.map((prop) => (
                  <GenericCard
                    image={prop.image}
                    network={prop.network}
                    token={prop.token}
                  />
                ))}
              </Flex>
            </VStack>
          </TabPanel>
          <TabPanel fontWeight="bold">
            <VStack spacing="10">
              <Heading size="md">Kusama</Heading>
              <Flex direction="row" gap="30px">
                {kusamaassets.map((prop) => (
                  <GenericCard
                    image={prop.image}
                    network={prop.network}
                    token={prop.token}
                  />
                ))}
              </Flex>
            </VStack>
          </TabPanel>
        </TabPanels>
      </Tabs>
    </>
  );
}

export { Portfolio };
