import React, { useState, useEffect } from 'react';
import ReactDOM from "react-dom";
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
import {
    ChakraProvider,
    Heading,
    Center,
    useDisclosure,
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    ModalBody,
    ModalCloseButton,
    Button,
    HStack,
  } from "@chakra-ui/react";
import { App } from "../../App";
  

function AccountButton () {

  const { isOpen, onOpen, onClose } = useDisclosure();

  const initialRef = React.useRef(null);
  const finalRef = React.useRef(null);
  
  const [accounts, setAccounts] = useState<string[]>([]);
  const [nameaccounts, setNameAccounts] = useState<any[]>([]);
  const [addressAccount, setAddressAccount]=useState("Connect");

  const enableExtension = async () => {
    await web3Enable('Your App Name');
    const accounts = await web3Accounts();
    setAccounts(accounts.map((account) => account.address));
    setNameAccounts(accounts.map((account) => account.meta.name))
    onOpen();
  };

  const updateStorage=()=>{

    const publicKey:any = localStorage.getItem("Publickey");
    setAddressAccount(publicKey);

    ReactDOM.render(
        <ChakraProvider>
          <App />
        </ChakraProvider>,
        document.getElementById("root")
      );

  }

  const updateAccount=()=>{

    const publicKey:any = localStorage.getItem("Publickey");
    setAddressAccount(publicKey);

  }

  useEffect(()=>{
    updateAccount()
  })

 
  

  return (
    <div>
      <Button borderRadius="20px" backgroundColor="orange.400" textColor="white" onClick={ enableExtension}>{(addressAccount != null) ? addressAccount : "Connect"}</Button>
      <Modal
        size="sm"
        initialFocusRef={initialRef}
        finalFocusRef={finalRef}
        isOpen={isOpen}
        onClose={onClose}
      >
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Select your Wallet : </ModalHeader>
          <ModalCloseButton />
          <ModalBody pb={8}>
          <HStack>
            <Center>
             <ol>
          {nameaccounts.map((name, index) => (
            <Heading key={name} size="lg" borderRadius="20px" textColor="orange.400">{name}</Heading>
          )
          )}
            </ol>
          {accounts.length > 0 && (
        <ol>
          {accounts.map((account, index) => (
            <Center key={ index} flexDirection="column" > 
            <Button key={account} border="4px" borderRadius="20px" backgroundColor="orange.400" textColor="white" onClick={()=> {localStorage.setItem("Publickey", account);  updateStorage(); onClose();}}>{account.slice(0, 5) + "..." + account.slice(-5)}</Button>
            </Center>
          )
          )}
        </ol>
      )}
      </Center>
      </HStack>
          </ModalBody>
          
          <Center>
            <ModalFooter>
            </ModalFooter>
          </Center>
        </ModalContent>
      </Modal>
    </div>
  );
};

export {AccountButton};