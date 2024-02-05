use std::{fs, path, vec};

#[tauri::command]
pub async fn create_folder(folder_name: String) -> path::PathBuf {
    let mut root_path: String = "../../".to_owned(); 
    let actual_path: String = folder_name.to_owned();
    root_path.push_str(&actual_path); // this will create a folder at "src-tauri"
    print!("{:?}", actual_path);
    match fs::create_dir(&actual_path){
        Ok(()) => {
            print!("Successfully created a folder named: {}", folder_name)
        },
        Err(_) => {

        }
    }
    path::PathBuf::from(root_path)
}

// TODO: Return RESULT to the frontend
#[tauri::command]
pub async fn get_the_directory(directory: String) -> Result<String, String> {
    Err(())
}

#[tauri::command]
pub async fn get_all_files() -> Vec<String>{
    let path = ".";
    let dir = fs::read_dir(path);
    let mut files: Vec<String> = Vec::new();
    match dir {
        Ok(entries) => {
            for entry in entries{
                match entry{
                    Ok(entry) => {
                        files.push(entry.file_name().to_str().unwrap().to_string());
                    },
                    Err(_) => {
                
                    }
                }
            }
        }, 
        Err(_) => {
            print!("Error on the dir!")
            
        }
    }
    files
}