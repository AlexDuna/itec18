// Login.js
import React, { useRef, useState } from 'react';
import './Login.css';
import { Canvas, useFrame } from '@react-three/fiber';
import { useGLTF, Stage} from '@react-three/drei';

function RotatingModel({ position, scale = 1 , path = "/book.glb"}) {
  const { scene } = useGLTF(path); // Asigură-te că e în /public
  const ref = useRef();
  let change = 0.00005;
  let accumchange = 0;
  let max = 0.01;
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
  console.log(state);
  return(
  <div className="login-page">
        { state ? (

         <div className="login-container">
        <h2>Login</h2>
        <form>
          <input type="text" placeholder="Username" />
          <input type="password" placeholder="Password" />
          <button className="submit" type="submit">Login</button>
          <button className="submit" type="submit" onClick={()=>setState(0)}>Already have an account?</button>
        </form>
        </div>
        ):(
          <div className="login-container">
        <h2>Login</h2>
        <form>
          <input type="text" placeholder="Username" />
          <input type="text" placeholder="Email" />
          <input type="password" placeholder="Password" />
          <button className="submit" type="submit">Login</button>
        </form>
        
        </div>
        )}
<div className='canvas-side'>
        <Canvas dpr={[1,2]} shadows camera={{ fov: 60 }} style={{ position: "absolute", top: 0, left: '30%', height: "100%", width: "100%" }}>
            <Stage environment={"sunset"}>
              <RotatingModel scale={1.0} />
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


