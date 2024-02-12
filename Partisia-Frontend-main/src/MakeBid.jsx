// MakeBid.js
import React, { useState } from 'react';
import './MakeBid.css';
import { Link, Route, Routes } from 'react-router-dom';
import List from './List';

function MakeBid({ onClose }) {
  const [bidAmount, setBidAmount] = useState('');
  const [description, setDescription] = useState('');

  const handleBidAmountChange = (e) => {
    setBidAmount(e.target.value);
  };

  const handleDescriptionChange = (e) => {
    setDescription(e.target.value);
  };

  const handleListBid = () => {
    // Implement logic to list bid
    console.log("Bid Amount:", bidAmount);
    console.log("Description:", description);
    // Close the MakeBid component
    return (<Routes>
      <Route exact path='/list' element={<List />} />
    </Routes>);
  };

  return (
    <div className="make-bid">
      <h2>Make Your Bid</h2>
      <label htmlFor="bidAmount">Bid Amount:</label>
      <input
        type="number"
        id="bidAmount"
        value={bidAmount}
        onChange={handleBidAmountChange}
      />
      <label htmlFor="description">Description:</label>
      <textarea
        id="description"
        value={description}
        onChange={handleDescriptionChange}
      />
      <Link to={"/list"} >List Bid</Link>
    </div>
  );
}

export default MakeBid;
