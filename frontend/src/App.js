import { BrowserRouter, Route, Routes } from 'react-router'
import Main from './Main.js'
import './Main.css'
import { useState } from 'react'

function App() {

  return (
    <div className="App">
      <BrowserRouter>
        <Routes>
            <Route path="/" element={<Main/>} />
        </Routes>
      </BrowserRouter>
    </div>
  )
  
}

export default App