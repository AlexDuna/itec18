import React from 'react';
import data from './components/fisier.json'; 

function DisplayFromJson() {
  return (
    <div>
      <h2>Sesiuni</h2>
      <ul>
        {data.sessions.map((session) => (
          <li key={session.id}>
            <strong>{session.title}</strong>
          </li>
        ))}
      </ul>

      <h2>Recomandări</h2>
      <ul>
        {data.recommended.map((recommendation) => (
          <li key={recommendation.id}>
            <strong>{recommendation.title}</strong>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default DisplayFromJson;
