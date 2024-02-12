// Home.js
import React, { useState } from 'react';
import './Home.css';
import { useNavigate } from 'react-router-dom';
import landing from "./assets/landingo.svg"
import useConnect from './hooks/useConnect';
//import {ethers} from "ethers";
// import { Button, Card } from "react-bootstrap";
// import "bootstrap/dist/css/bootstrap.min.css";

function Home() {

  const [connectButton, setConnectButton] = useState('Connect to MetaMask');
  const connect = useConnect();

  console.log(connect);

  const navigate = useNavigate();

  

  function handleListRoute() {
    navigate("/list")
  }
  function handleListBidRoute() {
    navigate("/list-bid")
  }
  return (<>
    {/* <Header isConnected={isConnected} connectButton={connectButton} handleConnect={handleConnect}/> */}
    <div className="home">
      <div className="home-img">
        <img src={landing}></img>
      </div>
      <div className="home-body">
        <div className="home-body-text">
          <div className="home-body-title">
            Lorem ipsum dolor, sit amet consectetur adipisicing elit. Odit, ab.
          </div>
          <div className="home-body-main-text">
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Eligendi illo autem quaerat optio, perferendis quo non maiores nulla repudiandae suscipit fugiat quisquam aspernatur officiis voluptatibus nisi dolor! Maiores, pariatur quia?
          </div>
        </div>
        <main className="content">
          {!connect.isConnected ? (
            <div className="centered-content">
              <button onClick={connect.handleConnect}>{connect.isConnected ? connect.account: "Connect Wallet"}</button>
            -</div>
          )
            : connect.isConnected && (
              <div className="centered-content">
                <button onClick={handleListRoute}>List of Listed Projects</button>
                <button onClick={handleListBidRoute}>List a Bid</button>
              </div>
            )}

        </main>
      </div>

    </div>
  </>

  );
}

export default Home;
