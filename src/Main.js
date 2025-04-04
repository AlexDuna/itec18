import React, { useEffect, useState } from 'react';
import {Canvas} from "@react-three/fiber";
import {useGLTF, Stage, PresentationControls} from "@react-three/drei";

function Model(props)
{
  const {scene} = useGLTF("/dacia.glb");
  return <primitive object={scene} {...props}/>
}
function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const param1 = searchParams.get('param1') || 'defaultA';
    const param2 = searchParams.get('param2') || 'defaultB';

    const apiUrl = `http://localhost:5000/testDOM?param1=${param1}&param2=${param2}`;

    fetch(apiUrl)
      .then((res) => res.json())
      .then((result) => setData(result))
      .catch((err) => console.error('Eroare la fetch:', err));
  }, []);

  return (
    <div className="container mt-5">
      <h1 className="text-primary">Test DOM Call</h1>
      <p>Rezultat de la server:</p>
      <pre>{JSON.stringify(data, null, 2)}</pre>
      {/* Afișează scena Three.js */}
      <Canvas dpr={[1,2]} shadows camera={{ fov: 60 }} style={{ position: "absolute", top: 0, left: 0, height: "100%", width: "100%" }}>
      <color attach="background" args={["#101010"]} />
        <PresentationControls speed={1.5} global zoom={0.5} polar={[-0.01, Math.PI / 4]}>
          <Stage environment={"sunset"}>
            <Model scale={1.0} />
         </Stage>
        </PresentationControls>
      </Canvas>
    </div>
    
  );
}

export default App;
