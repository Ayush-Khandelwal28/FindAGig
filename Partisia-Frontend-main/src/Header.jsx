// Header.js
import React from 'react';
import useConnect from './hooks/useConnect';

function Header() {
  const connect = useConnect();
  return (
    <header className="header">
      <h1>FindAGig</h1>
      <div className="account-info">
        {
          connect.isConnected && <button className="connect_button">{connect.connectButton}{' -->'}</button>
        }
        {
          !connect.isConnected &&
          <button className="connect_button" onClick={connect.handleConnect}>{connect.connectButton}</button>
        }
      </div>
    </header>
  );
}

export default Header;
