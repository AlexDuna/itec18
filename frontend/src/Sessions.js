// src/components/Sessions.js
import React, {useState} from 'react';
import DisplayFromJson from './DisplayFromJson';
import './Sessions.css';
import Card from 'react-bootstrap/Card';
import { Button } from 'react-bootstrap';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import {Link} from 'react-router-dom';
import { FaRocketchat } from "react-icons/fa";
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';


function Sessions() {
    const [isOpen, setIsOpen] = useState(false);

    const toggleChat = () => {
        setIsOpen(prevState => !prevState);
    };

  return (
    <div>

      <div className='right-sidebar'>
            <Nav defaultActiveKey="/sessions" className="flex-column">
                <Nav.Link onClick={toggleChat}>
                    <FaRocketchat id="chat-icon" size={40} />
                    <span>Chat</span>
                </Nav.Link>
            </Nav>
      </div>
      <div className={`chat-section ${isOpen ? 'open' : ''}`}>
                <div className="chat-content">
                    <h2>Chat Section</h2>
                    <p>Aici se va încărca chatul sau orice alt conținut dorești.</p>
                </div>
            </div>

      <div className='card'>
      <Row>
      <Col>
      <h1>Recommended</h1>
      <Card className='hover-card' style={{ width: '100%', height:'300px' }}>
      <Card.Body>
        <Card.Title>Card Title</Card.Title>
        <Card.Text>
          Some quick example text to build on the card title and make up the
          bulk of the card's content.
        </Card.Text>
        <Button variant="dark">Go somewhere</Button>
      </Card.Body>
        </Card>
    </Col>
    <Col>
    <h1>Sessions</h1>
      <Card className='hover-card' style={{ width: '100%', height:'300px' } }>
      <Card.Body>
        <Card.Title>Card Title</Card.Title>
        <Card.Text>
          Some quick example text to build on the card title and make up the
          bulk of the card's content.
        </Card.Text>
        <Button variant="dark">Go somewhere</Button>
      </Card.Body>
        </Card>
    </Col>
    </Row>
    </div>
    
    </div>
  );
}

export default Sessions;
