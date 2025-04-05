// Login.js
import React, { useRef, useState } from 'react';
import './Login.css';
import { Canvas, useFrame } from '@react-three/fiber';
import { useGLTF, Stage} from '@react-three/drei';

function RotatingModel({ position, scale = 1 , path = "/book.glb"}) {
  const { scene } = useGLTF(path); // Asigură-te că e în /public
  const ref = useRef();
  let change = 0.0001;
  let accumchange = 0;
  let max = 0.02;
  useFrame(() => {
    if (ref.current) {
      ref.current.position.y += change;
      accumchange += change;
    }
    if(accumchange > max || accumchange < -max) {
      accumchange = 0;
      change *= -1;
    }
  });

  return <primitive object={scene} position={position} scale={scale} ref={ref} />;
}

const Page = () => {
  let [state, setState] = useState(1);
  let [username, setUsername] = useState("");
  let [email, setEmail] = useState("");
  let [password, setPassword] = useState("");

  console.log(state);
  const handleForm = (e) => {
    e.preventDefault();
    console.log(e.defaultPrevented);
  }
  return(
  <div className="login-page">
        { state ? (

         <div className="login-container">
        <h2>Log In</h2>
        <div className="formdiv">
          <input type="text" value={username} onChange={(e)=>setUsername(e.target.value)} placeholder="Username" />
          <input type="password" value={password} onChange={(e)=>setPassword(e.target.value)} placeholder="Password" />
          <button className="submit" onClick={()=>{console.log(password)}}>Login</button>
          <button type="button"  value={state} onClick={()=>setState(0)}>Do not have an account?</button>
        </div>
        </div>
        ):(
          <div className="login-container">
        <h2>Sign Up</h2>
        <div className="formdiv">
          <input type="text" value={username} onChange={(e)=>setUsername(e.target.value)}  placeholder="Username" />
          <input type="text" value={email} onChange={(e)=>setEmail(e.target.value)} placeholder="Email" />
          <input type="password" value={password} onChange={(e)=>setPassword(e.target.value)}  placeholder="Password" />
          <button className="submit" type="submit">Login</button>
          
          <button type="button"  value={state} onClick={()=>setState(1)}>Already have an account?</button>
        </div>
        
        </div>
        )}
    <div className='canvas-side'>
        <Canvas dpr={[1,2]} shadows camera={{ fov: 20 }} style={{ position: "absolute", top: 0, left: '20%', height: "100%", width: "100%" }}>
            <Stage environment={"sunset"}>
              <RotatingModel scale={2.0} />
           </Stage>
        </Canvas>
       </div>
    </div>
  );
}
export default function Login() {
  console.log("test");
  return (
    <>
   <Page/>
    </>
  );
}


