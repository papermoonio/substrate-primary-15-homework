import React, { useState } from "react";
import {
  Center,
  Box,
  useDisclosure,
  FormLabel,
  FormControl,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  Button,
  VStack,
} from "@chakra-ui/react";

function AddAccountModal({ valueAddress }) {
  const { isOpen, onOpen, onClose } = useDisclosure();

  const initialRef = React.useRef(null);
  const finalRef = React.useRef(null);

  return (
    <>
      <Button
        colorScheme="red"
        borderRadius="20px"
        onClick={onOpen}
        className="center"
      >
        {" "}
        Add new account
      </Button>
      <Modal
        size="lg"
        initialFocusRef={initialRef}
        finalFocusRef={finalRef}
        isOpen={isOpen}
        onClose={onClose}
      >
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Your Public Address is : </ModalHeader>
          <ModalCloseButton />
          <ModalBody pb={8}>
            <FormControl mt={6}>
              <FormLabel>Address: </FormLabel>
              <p> {valueAddress}</p>
            </FormControl>
          </ModalBody>
          <Center>
            <ModalFooter>
              <VStack>
                <Button colorScheme="blue" borderRadius="20px">
                  <a href="./Portfolio">Add address</a>
                </Button>
              </VStack>
            </ModalFooter>
          </Center>
        </ModalContent>
      </Modal>
    </>
  );
}

export { AddAccountModal };
