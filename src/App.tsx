import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog, fs } from "@tauri-apps/api";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState([]);
  

  async function print_something(){
    setGreetMsg(await invoke("get_all_files"));
    console.log(greetMsg);
  }

  

  const [folderDirectory, setFolderDirectory] = useState<string | string[] | null>("");
  
  // Get the directory for where the file will be sorted
  async function select_directory(){
    const directory = await dialog.open({directory: true})
    await setFolderDirectory(directory)
    console.log(`You've picked: ${directory}`);
  }

  // Send directory to the back
  async function send_back_directory(){
    await invoke("get_the_directory", {directory: folderDirectory})
    .then((message) => {
      console.log(message);
      
    })
    .catch((err) => {
      console.log(err);
    })
  } 

  useEffect(() => {

    // If the directory exists, proceed on sending to the backend
    if(folderDirectory){
      send_back_directory()
    }
    
  }, [folderDirectory])

  return (
    <div>
      <h1> File Manager </h1>
      <div>
        Folder Directory: {folderDirectory}
      </div>
      <div> 
        <button onClick={select_directory} />
      </div>
    </div>
  );
}


export default App;
