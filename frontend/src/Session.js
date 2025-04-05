import React, {useState} from 'react';
import './Sessions.css';
import Card from 'react-bootstrap/Card';
import { Button } from 'react-bootstrap';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import {Link} from 'react-router-dom';
import { FaRocketchat } from "react-icons/fa";
import { CiMail } from "react-icons/ci";
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import { IoMailOpenOutline } from "react-icons/io5";
import { MdMailOutline } from "react-icons/md";


export default function Session() {
const [isOpen, setIsOpen] = useState(false);

    const toggleChat = () => {
        setIsOpen(prevState => !prevState);
    };

    const getThemes = () => {
        return {
          themes: [
            {
              id: "Lecture 1",
              title: "React Basics",
              description: "Learn the basics of React.",
              owner: "Mic",
            }     
          ],
        };
      };

    return (
<>
<div className={`right-sidebar ${isOpen ? 'shifted' : ''}`}>
            <Nav defaultActiveKey="/sessions" className="flex-column">
                <Nav.Link onClick={toggleChat}>
                    { isOpen ? (
                        <IoMailOpenOutline id="chat-icon" size={30} />
                    ):(
                        <MdMailOutline id="chat-icon" size={30} />
                    )}
                </Nav.Link>
            </Nav>
            
        <div className={`chat-section ${isOpen ? 'open' : ''}`}>
                <div className="chat-content">
                    <h2>Chat</h2>
                    <div className="chatbar" placeholder='Chat here'>
                     <input className="chatin" placeholder='Chat...'/>
                    </div>
            </div>
        </div>
        </div>
    <Row>
          <Col>
          <h1>Recommended</h1>
        </Col>
    </Row>

</>

    );

}