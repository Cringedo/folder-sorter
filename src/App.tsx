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

  

  const [folderDirectory, setFolderDirectory] = useState<string | string[] | null>("Insert a directory");
  
  // Get the directory for where the file will be sorted
  async function select_directory(){
    const directory = await dialog.open({directory: true})
    await setFolderDirectory(directory)
    console.log(`You've picked: ${directory}`);
  }

  const [sortBy, setSortBy] = useState<string | String | null>()

  // Send directory to the back
  async function send_back_directory(){
    await invoke("get_the_directory", {directory: folderDirectory, sortType: sortBy})
    .then((message) => {
      console.log(message);
      
    })
    .catch((err) => {
      console.log(err);
    })
  } 

  const [testi, settesti] = useState("");
  const test = () => {
    console.log(testi)
  }


  useEffect(() => {

    // If the directory exists, proceed on sending to the backend
    // if(folderDirectory){
    //   send_back_directory()
    // }

  }, [folderDirectory])

  return (
    <div className="main_div">
      <div className="header_div">
        <h1 className="header_text"> Cringedo's File Sorter </h1>
      </div>
     
      <div className="side_div">
        <div className="side_main_div">

          <p className="side_components"> Folder Directory</p>  
          <button id="dir-button"  onClick={select_directory} hidden/>
          <label htmlFor='dir-button' className="side_components_border"> {folderDirectory}</label>

          <p className="side_components"> Sort by  </p>
          <div className="radio-div">
            <input type="radio" id="radio-year" name="sortby" onChange={() => setSortBy("year")} hidden/> <label id="label-year" htmlFor="radio-year"> Year </label>
            <input type="radio" id="radio-filetype" name="sortby" onChange={() => setSortBy("filetype")} hidden/> <label id="label-filetype" htmlFor="radio-filetype"> File Type </label>
          </div>
          
          <p className="side_components" />
          <button className="button-submit" onClick={send_back_directory}> Sort </button>
        </div>
      </div>
    </div>
    
  );
}


export default App;
