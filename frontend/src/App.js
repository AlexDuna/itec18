import { BrowserRouter, Route, Routes } from 'react-router'
import Main from './Main.js'
import Login from "./Login.js"
import './Main.css'
import { useState } from 'react'
import act from 'react'

function App() {

  return (
    <div className="App">
      <BrowserRouter>
        <Routes>
            <Route path="/" element={<Main/>} />
            <Route path="/login" element={<Login />} />
        </Routes>
      </BrowserRouter>
    </div>
  )
  
}

export default App
