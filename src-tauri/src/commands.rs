extern crate chrono;
use std::{fmt::format, fs::{self, DirEntry}, path, time::SystemTime, vec};
use chrono::offset::Utc;
use chrono::DateTime;

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

struct SendFile {
    msg: String,
    files: Vec<FileInfo>,
    years: SystemTime
}

#[derive(Debug)]
pub struct FileInfo {
    name: String,
    path: String,
    year: String
}

// TODO: Return RESULT to the frontend
#[tauri::command]
pub async fn get_the_directory(directory: String) -> Result<String, String> {
    let directory_log: &String = &directory.clone();
    if directory.is_empty() {
        let err_msg: String = format!("It looks like the directory sent is empty! Directory: {}", directory_log);
        Err(err_msg.into())
    }
    else {
        let files: Vec<FileInfo> = get_all_files(directory).await;
        let years: Vec<String> = get_all_years(&files).await;
        let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}\nYears: {:?}", directory_log, files, years);

        // let files_str: String = files.join(", ");
        // let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}", directory_log, files_str);
        // let ok_msg: String = format!("[🦀] Success!");
        Ok(ok_msg.into())
    }
}

#[tauri::command]
pub async fn get_all_files(path: String) -> Vec<FileInfo>{
    // let path = ".";
    let dir = fs::read_dir(path.clone());
    let mut files: Vec<FileInfo> = Vec::new();
    match dir {
        Ok(entries) => {
            for entry in entries{
                match entry{
                    Ok(entry) => {
                        
                        // files.push(file_path.clone());
                        // let example_dir = format!("{}/{}", path.clone(), "test");
                        // let _ = fs::create_dir(example_dir);
                        
                        let filename = entry.file_name().to_str().unwrap().to_string();
                       
                        let datetime: DateTime<Utc> = entry.metadata().unwrap().created().unwrap().into();
                        let year: String = datetime.format("%Y").to_string();
            
                        let file = FileInfo {
                            name: filename.clone(),
                            path: path.clone(),
                            year: year // <--- probably kena tengok ni kemudian
                        };       

                        files.push(file)
                        
                        // TODO: 
                        // 1. Get all the file actual path together with the file name
                        // 2. Get all the years from the year
                        // 3. Return it into a SendFile contains directory, files, years
                        
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

pub async fn get_all_years(files: &Vec<FileInfo>) -> Vec<String>{
    let mut available_years: Vec<String> = Vec::new();
    for file in files {
        let year = file.year.clone();
        if !available_years.contains(&year) {
           available_years.push(year) 
        }
    }
    available_years
}

// TEMP!
// #[tauri::command]
// pub async fn get_all_files(path: String) -> Vec<String>{
//     // let path = ".";
//     let dir = fs::read_dir(path.clone());
//     let mut files: Vec<String> = Vec::new();
//     match dir {
//         Ok(entries) => {
//             for entry in entries{
//                 match entry{
//                     Ok(entry) => {
//                         let file_path = entry.file_name().to_str().unwrap().to_string();
//                         files.push(file_path.clone());
//                         let example_dir = format!("{}/{}", path.clone(), "test");
//                         let _ = fs::create_dir(example_dir);

                    
//                         // TODO: 
//                         // 1. Get all the file actual path together with the file name
                        
//                         // 2. Get all the years from the year
//                         // 3. Return it into a SendFile contains directory, files, years
                        
//                     },
//                     Err(_) => {
                
//                     }
//                 }
//             }
//         }, 
//         Err(_) => {
//             print!("Error on the dir!")
            
//         }
//     }
//     files
// }