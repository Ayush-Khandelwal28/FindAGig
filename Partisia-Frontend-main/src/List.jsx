// List.js
import React from 'react';
import { Link } from 'react-router-dom';
import './List.css';
import Popup from 'reactjs-popup';
import './reactjs-popup.css';
import MakeBid from './MakeBid';

function List() {
  // Assume projects is an array containing project information
  const projects = [
    {
      id: 1,
      datePublished: "2024-02-07",
      lastDateToBid: "2024-02-14",
      description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla convallis risus et maximus dapibus.",
    },
    {
      id: 2,
      datePublished: "2024-02-08",
      lastDateToBid: "2024-02-15",
      description: "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.",
    }
  ];

  return (
    <div className="list">
      <h2>List of Projects Available to Bid</h2>
      {projects.length === 0 ? (
        <p>No Projects available to bid, Come Back Later</p>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Project ID</th>
              <th>Date Published</th>
              <th>Last Date to Bid</th>
              <th>Description</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {projects.map(project => (
              <tr key={project.id}>
                <td>{project.id}</td>
                <td>{project.datePublished}</td>
                <td>{project.lastDateToBid}</td>
                <td>
                  <Popup trigger={<button>View Description</button>}>
                    <div className="description-content">Heyy</div>
                  </Popup>
                </td>
                <td><Link to={`/bid`} className="bid-button">Bid</Link></td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      <Link to="/" className="back-button">Back to Home</Link>
    </div>
  );
}

export default List;
