// App.js
import './App.css';
import Home from './home';
import { Route, Routes } from 'react-router-dom';
import List from './List';
import YourBids from './YourBids';
import React, { useState } from 'react'; 
import ListBid from './ListBid';
import YourListing from './YourListing';
import Header from './Header';
import MakeBid from './MakeBid';

function App() {

  return (
    <div>
      <Header/>
      <Routes>
      <Route exact path="/" element={<Home />} />
      <Route exact path="/list" element={<List />} />
      <Route exact path="/yourbid" element={<YourBids />} />
      <Route exact path="/list-bid" element={<ListBid />} />
      <Route exact path='/your-listing' element={<YourListing />} />
      <Route exact path='/bid' element={<MakeBid />} / > 
      {/* Add other routes here if necessary */}
    </Routes>
    </div>
      
  );
}

export default App;
