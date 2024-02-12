// ListBid.js
import React, { useState } from 'react';
import { useNavigate  } from 'react-router-dom'; // Import useHistory hook
import './ListBid.css';

function ListBid() {
  const [projectName, setProjectName] = useState('');
  const [projectDescription, setProjectDescription] = useState('');
  const history = useNavigate(); // Get history object from React Router

  const handleProjectNameChange = (e) => {
    setProjectName(e.target.value);
  };

  const handleProjectDescriptionChange = (e) => {
    setProjectDescription(e.target.value);
  };

  const handleListProject = () => {
    history('/');
  };

  return (
    <div className="list-bid">
      <h2>List a Project</h2>
      <div className="form-group">
        <label htmlFor="projectName">Name of the Project:</label>
        <input
          type="text"
          id="projectName"
          value={projectName}
          onChange={handleProjectNameChange}
        />
      </div>
      <div className="form-group">
        <label htmlFor="projectDescription">Description of the Project:</label>
        <textarea
          id="projectDescription"
          value={projectDescription}
          onChange={handleProjectDescriptionChange}
        />
      </div>
      <button onClick={handleListProject}>List Project</button>
    </div>
  );
}

export default ListBid;
