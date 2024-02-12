// YourBids.js
import React from 'react';
import './YourBids.css';

function YourBids() {
  // Assuming you have data for ongoing and completed bids
  const ongoingBids = [
    { id: 1, name: "Project 1", amount: 100, status: "Successful" },
    { id: 2, name: "Project 2", amount: 150, status: "Rejected" },
  ];

  const completedBids = [
    { id: 3, name: "Project 3", amount: 200, status: "Successful" },
    { id: 4, name: "Project 4", amount: 120, status: "Rejected" },
    { id: 5, name: "Project 5", amount: 180, status: "Successful" },
  ];

  return (
    <div className="your-bids">
      <h2>Ongoing Bids</h2>
      <table className="bid-table">
        <thead>
          <tr>
            <th>Project ID</th>
            <th>Project Name</th>
            <th>Bid Amount</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {ongoingBids.map(bid => (
            <tr key={bid.id}>
              <td>{bid.id}</td>
              <td>{bid.name}</td>
              <td>${bid.amount}</td>
              <td className={bid.status === "Successful" ? "successful" : "rejected"}>{bid.status}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2>Completed/Expired Bids</h2>
      <table className="bid-table">
        <thead>
          <tr>
            <th>Project ID</th>
            <th>Project Name</th>
            <th>Bid Amount</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {completedBids.map(bid => (
            <tr key={bid.id}>
              <td>{bid.id}</td>
              <td>{bid.name}</td>
              <td>${bid.amount}</td>
              <td className={bid.status === "Successful" ? "successful" : "rejected"}>{bid.status}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default YourBids;
