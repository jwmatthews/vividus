import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { ask } from '@tauri-apps/api/dialog';


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [imagePath, setImagePath] = useState("");
  const [images, setImages] = useState([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.error("Greet");
    setGreetMsg(await invoke("greet", { name }));
  }

  async function get_images() {
    console.error("Inside get_images()");
    console.error(imagePath);
    setImages(await invoke("list_images", {imagePath}));
  }

  async function testyes() {
    const yes = await ask('Are you sure?', 'Tauri');
    const yes2 = await ask('This action cannot be reverted. Are you sure?', {title: 'Tauri', type: 'warning'});
    console.error("yes is " + yes);
    console.error("yes2 is " + yes2);
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_images();
        }}
      >
        <input
          id="getimages-input"
          onChange={(e) => setImagePath(e.currentTarget.value)}
          placeholder="/Users/jmatthews/Pictures/Wulfgar"
        />
        <button type="submit">Update image path</button>
      </form>

      {/* <ul>
        {images.map((image, index) => (
          <li key={index}>{image}</li>
        ))}
      </ul> */}

      <div>
        {images.map((image, index) => (
          <img src={image} alt="{index}" width="500" height="600"/>
        ))}
      </div>

      {/*  <p>{images}</p> */}
      <button onClick={testyes}>Yes</button>


    </div>
  );
}

export default App;
