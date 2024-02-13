import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog, fs } from "@tauri-apps/api";
import "./App.css";
import Modal from "./ErrModel.tsx";;

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

  const [sortBy, SetSortBy] = useState<string | String | null>()
  const [progress, SetProgress] = useState<string | string[] | null> ("Sort")
  const [isSorting, SetIsSorting] = useState(false);

  // Send directory to the back
  async function send_back_directory(){
    if(check_if_input_error() == false && progress != "Sorting in progress.."){
      SetIsSorting(true);
      SetProgress("Sorting in progress..")
      await invoke("get_the_directory", {directory: folderDirectory, sortType: sortBy})
      .then((message) => {
        console.log(message);
        SetProgress("Sort Finished")
        SetIsSorting(false)
      })
      .catch((err) => {
        console.log(err);
      })
    }
  } 

  const [showModal, SetShowModal] = useState(false);
  const [err, SetErr] = useState("ERR: Invalid Directory, please resubmit");

  const check_if_input_error = (): boolean => {
    let is_error: boolean = false;
    SetErr("Looks fine!")
    if(!sortBy || !folderDirectory || folderDirectory == "Insert a directory"){
      SetErr("ERR: Invalid Directory.\nPlease double check the directory.");
      SetProgress("Sort")
      console.log(`It looks like that:\n${sortBy}\n${folderDirectory}`);
      is_error = true;
    }
    SetShowModal(is_error);
    return is_error;
  };

  const closeModal = () => {
      SetShowModal(false);
  };

  
  // TODO: updates the text inside button to match the current progress
  const showProgess = () => {
    if(!err.match("Looks fine!")){
      SetProgress("Sort")
      SetIsSorting(false)
      console.log(isSorting);
      return
    }

  }

  useEffect(() => {

    // If the directory exists, proceed on sending to the backend
    // if(folderDirectory){
    //   send_back_directory()
    // }

    showProgess()
   
    

  }, [folderDirectory, err, isSorting])

  return (
    <div className="main_div">
      <div>
            <Modal show={showModal} handleClose={closeModal}>
                <p>{err}</p>
            </Modal>
        </div>
        <div>
            <Modal show={showModal} handleClose={closeModal}>
                <p>{err}</p>
            </Modal>
        </div>
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
            <input type="radio" id="radio-year" name="sortby" onChange={() => SetSortBy("year")} hidden/> <label id="label-year" htmlFor="radio-year"> Year </label>
            <input type="radio" id="radio-filetype" name="sortby" onChange={() => SetSortBy("filetype")} hidden/> <label id="label-filetype" htmlFor="radio-filetype"> File Type </label>
          </div>
          
          <p className="side_components" />
          <div className="div-submit">
            <button className="button-submit" onClick={send_back_directory} disabled={isSorting}> {progress}</button>
            <p className="text-progress">test</p>
          </div>
        </div>
      </div>
    </div>
    
  );
}


export default App;
