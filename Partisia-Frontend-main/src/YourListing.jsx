// YourListing.js
import React from 'react';
import './YourListing.css';

function YourListing() {
  // Mock data for ongoing, completed bids, completed projects, and disputes
  const ongoingBids = [
    { projectId: 1, projectName: "Project 1", highestBid: 200 },
    { projectId: 2, projectName: "Project 2", highestBid: 300 },
  ];

  const completedBids = [
    { projectId: 3, projectName: "Project 3", highestBid: 400 },
    { projectId: 4, projectName: "Project 4", highestBid: 250 },
  ];

  const completedProjects = [
    { projectId: 5, projectName: "Project 5", highestBid: 350 },
    { projectId: 6, projectName: "Project 6", highestBid: 500 },
  ];

  const disputes = []; // Empty array for simplicity

  return (
    <div className="your-listing">
      <h2>Ongoing Bids</h2>
      <table className="listing-table">
        <thead>
          <tr>
            <th>Project ID</th>
            <th>Project Name</th>
            <th>Highest Bid</th>
          </tr>
        </thead>
        <tbody>
          {ongoingBids.map(bid => (
            <tr key={bid.projectId}>
              <td>{bid.projectId}</td>
              <td>{bid.projectName}</td>
              <td>${bid.highestBid}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2>Completed Bids</h2>
      <table className="listing-table">
        <thead>
          <tr>
            <th>Project ID</th>
            <th>Project Name</th>
            <th>Highest Bid</th>
          </tr>
        </thead>
        <tbody>
          {completedBids.map(bid => (
            <tr key={bid.projectId}>
              <td>{bid.projectId}</td>
              <td>{bid.projectName}</td>
              <td>${bid.highestBid}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2>Completed Projects</h2>
      <table className="listing-table">
        <thead>
          <tr>
            <th>Project ID</th>
            <th>Project Name</th>
            <th>Highest Bid</th>
          </tr>
        </thead>
        <tbody>
          {completedProjects.map(project => (
            <tr key={project.projectId}>
              <td>{project.projectId}</td>
              <td>{project.projectName}</td>
              <td>${project.highestBid}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2>Dispute Section</h2>
      {disputes.length === 0 ? (
        <p>No Dispute Requests Currently</p>
      ) : (
        <table className="listing-table">
          <thead>
            <tr>
              <th>Project ID</th>
              <th>Dispute ID</th>
            </tr>
          </thead>
          <tbody>
            {disputes.map(dispute => (
              <tr key={dispute.disputeId}>
                <td>{dispute.projectId}</td>
                <td>{dispute.disputeId}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}

export default YourListing;
