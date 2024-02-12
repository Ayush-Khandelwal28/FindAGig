import React, { createContext, useState } from "react";

const ConnectContext = createContext({
    account: '',
    setAccount: () => { },
    isConnected: false,
    setIsConnected: () => { },
    connectButton : "",
    setConnectButton: () => {},
    handleConnect: () => {}
});

export function ConnectContextProvider({ children }) {
    const [account, setAccount] = useState('');
    const [isConnected, setIsConnected] = useState(false);
    const [connectButton, setConnectButton] = useState('Connect to Metamask')

    const handleConnect = async () => {
        const accounts = await window.ethereum.request({ method: "eth_requestAccounts" });
        setAccount(accounts[0]);
        setConnectButton(accounts[0].slice(0, 5) + "..." + accounts[0].slice(-5));
        setIsConnected(true);
        // setConnectButton("Connecting...");
        return;
      }

    return (
        <ConnectContext.Provider value={{ account, setAccount, isConnected, setIsConnected, connectButton, setConnectButton, handleConnect }}>
            {children}
        </ConnectContext.Provider>
    );
}

export default ConnectContext;