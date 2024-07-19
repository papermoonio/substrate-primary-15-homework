import { useState } from "react";
import { CopyToClipboard } from "react-copy-to-clipboard";
import {
  useToast,
  useDisclosure,
  Button,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalCloseButton,
  HStack,
  Heading,
} from "@chakra-ui/react";

import { MdContentCopy } from "react-icons/md";

function RecieveModal() {
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const publicKey = localStorage.getItem("Publickey");
  const [text, setText] = useState("");
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    setCopied(true);
    const contenido = document.querySelector<HTMLInputElement>("#input")?.value;
    if (contenido) {
      navigator.clipboard.writeText(contenido);
    }
  };

  return (
    <>
      <Button variant="ghost" colorScheme="green" onClick={onOpen}>
        Recieve
      </Button>

      <Modal size="lg" onClose={onClose} isOpen={isOpen} isCentered>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Your address is: </ModalHeader>
          <HStack>
            <Heading id="input" size="sm" className="center">
              {publicKey}
            </Heading>
            <CopyToClipboard text={text} onCopy={handleCopy}>
              <Button variant="ghost" colorScheme="green">
                <MdContentCopy />
              </Button>
            </CopyToClipboard>
          </HStack>
          {copied
            ? toast({
                title: "Copied",
                duration: 3000,
                status: "success",
                isClosable: true,
              })
            : null}
          <ModalCloseButton />
        </ModalContent>
      </Modal>
    </>
  );
}

export { RecieveModal };
