extern crate chrono;
use std::{fmt::format, fs::{self, DirEntry}, path, time::SystemTime, vec};
use chrono::offset::Utc;
use chrono::DateTime;
use tauri::api::file;
use std::path::Path;

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

#[derive(Debug)]
enum FileType {
    Image, Video, Music, Executable, Other
}

#[derive(Debug)]
pub struct FileInfo {
    name: String,
    path: String,
    year: String,
    file_type: FileType
}

// TODO: Return RESULT to the frontend
#[tauri::command]
pub async fn get_the_directory(directory: String, sort_type: String) -> Result<String, String> {
    let directory_log: &String = &directory.clone();
    let sort_type_log: &String = &sort_type.clone();

    if directory.is_empty() {
        let err_msg: String = format!("It looks like the directory sent is empty! Directory: {}", directory_log);
        Err(err_msg.into())
    }
    else {
        let files: Vec<FileInfo> = get_all_files(directory).await;
        let years: Vec<String> = get_all_years(&files).await;
        
        // Sort based on the sort type given from the frontend
        match sort_type_log.as_str() {
            "year" => {
                create_dir_by_years(directory_log, &years).await;
                copy_files_into_folders(directory_log, &files).await;
            },
            "filetype" => {

            },
            _ => {
                // If 'nothing' is somehow get into the this, just use "year"
                create_dir_by_years(directory_log, &years).await;
                copy_files_into_folders(directory_log, &files).await;
            }
            
        }



        // let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}\nYears: {:?}", directory_log, files, years);
        let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}\nYears: {:?}", directory_log, files, years);

        // let files_str: String = files.join(", ");
        // let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}", directory_log, files_str);
        // let ok_msg: String = format!("[🦀] Success!");
        Ok(ok_msg.into())
    }
}

#[tauri::command]
pub async fn get_all_files(path: String) -> Vec<FileInfo>{
    let dir = fs::read_dir(path.clone());
    let mut files: Vec<FileInfo> = Vec::new();
    match dir {
        Ok(entries) => {
            for entry in entries{
                match entry{
                    Ok(entry) => {
                        
                        // Get the file name
                        let filename = entry.file_name().to_str().unwrap().to_string();
                       
                        // Get the file year
                        let datetime: DateTime<Utc> = entry.metadata().unwrap().created().unwrap().into();
                        let file_year: String = datetime.format("%Y").to_string();
                        
                        // Get the file format/type  
                        let file_ext: &str = filename.split(".").last().unwrap();
                        let file_type: FileType;
                        
                        // 🚧TODO: Update this to possible file format
                        match file_ext {
                            "wav" => file_type = FileType::Music,
                            "mp3" => file_type = FileType::Video,
                            ".png" => file_type = FileType::Image,
                            _ => file_type = FileType::Other
                        }
                        
                        // Compile them into a struct 
                        let file = FileInfo {
                            name: filename.clone(),
                            path: path.clone(),
                            year: file_year, // <--- probably kena tengok ni kemudian
                            file_type: file_type
                        };       

                        files.push(file)
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

pub async fn create_dir_by_years(directory: &String, years: &Vec<String>) {
    for year in years {
        let folder_dir = format!("{}/{}", directory, year);
        match fs::create_dir(folder_dir.clone()){
            Ok(_) => print!("Success! Years for folders have been created!"),
            Err(e) => print!("{}", e)
        }
    }
}

// Copy to the files into the folders
pub async fn copy_files_into_folders(directory: &String, files: &Vec<FileInfo>){
    for file in files{
        let old_folder_dir = Path::new(directory).join(&file.name);
        let new_folder_dir = Path::new(directory).join(&file.year).join(&file.name);
        let _ = fs::copy(old_folder_dir, new_folder_dir);
    }
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