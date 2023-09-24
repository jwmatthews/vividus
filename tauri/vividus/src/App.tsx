import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { ask } from '@tauri-apps/api/dialog';
import { readBinaryFile, BaseDirectory } from '@tauri-apps/api/fs';
//import { pictureDir } from '@tauri-apps/api/path';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [imagePath, setImagePath] = useState("");
  const [images, setImages] = useState([]);
  const [testImage, setTestImage] = useState("");

  function hexToBase64(str: any) {
    return btoa(String.fromCharCode.apply(null, str.replace(/\r|\n/g, "").replace(/([\da-fA-F]{2}) ?/g, "0x$1 ").replace(/ +$/, "").split(" ")));
  }

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

  async function loadTestImage() {
    console.error("fetchTestImage");
    //const pictureDirPath = await pictureDir();
    //const myPicPath = await join(pictureDirPath, 'therapysword.jpeg');

    const contents = await readBinaryFile('therapysword.jpeg', { dir: BaseDirectory.Picture });
    //setTestImage( await  'data:image/jpeg;base64,' + hexToBase64(contents) );
    setTestImage( await  'data:image/jpeg;,' + contents );
  }


  // Example from: https://tauri.app/v1/api/js/fs/#readbinaryfile
  // Read the image file in the `$RESOURCEDIR/avatar.png` path
  // const contents = await readBinaryFile('avatar.png', { dir: BaseDirectory.Resource });
  //
  // Issue:  WebView is not able to read the entire local filesystem.
  //        It can only read the contents of the resource directory.
  //        The resource directory is the directory where the executable is located.
  // Is it possible to grant the WebView process to the entire local filesystem?
  // If not, I could copy images to Resource Directory
  //
  // Or...
  // Use the fs API to read files in a directory
  // Iterate over and use readBinaryFile, that will return the contents via IPC comm to webview process
  // In Webview process display the binary image data
  // https://stackoverflow.com/questions/14915058/how-to-display-binary-data-as-image-extjs-4
  // https://stackoverflow.com/questions/18650168/convert-blob-to-base64


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
        <p>Test area to load Ferris image</p>
        <img src={testImage} alt="Ferris" width="500" height="600"/>
        <button onClick={loadTestImage}>Load Test Image</button>
      </div>
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
