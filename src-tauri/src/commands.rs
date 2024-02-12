extern crate chrono;
use std::{fmt::format, fs::{self, DirEntry}, path, time::SystemTime, vec};
use chrono::offset::Utc;
use chrono::DateTime;
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

#[derive(Debug, Clone)]
enum FileType {
    Image, Video, Music, Executable, Other
}

impl FileType {
    fn as_str(&self) -> &str {
        match &self {
            FileType::Music => "music",
            FileType::Video => "video",
            FileType::Image => "image",
            FileType::Executable => "executable",
            FileType::Other => "other"
        }
    }
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
        let sort_details: Vec<String> = get_all_sort_values(&files, &sort_type_log.as_str()).await;
        create_dir_by_sort_values(directory_log, &sort_details).await;
        copy_files_into_folders(directory_log, &files, &sort_type).await;
            



        // let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}\nYears: {:?}", directory_log, files, years);
        let ok_msg: String = format!("[🦀] Success!\nDirectory: {}\nFiles: {:?}\nSort Details: {:?}", directory_log, files, sort_details);

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
                            ".exe" => file_type = FileType::Executable,
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

pub async fn get_all_sort_values(files: &Vec<FileInfo>, sort_type: &str) -> Vec<String>{
    let mut available_sort_values: Vec<String> = Vec::new();
    match sort_type {
        "year" => {
            for file in files {
                let year = file.year.clone();
                if !available_sort_values.contains(&year) {
                   available_sort_values.push(year) 
                }
            }
        },
        "filetype" => {
            for file in files {
                let file_type = file.file_type.as_str().to_string();
                if !available_sort_values.contains(&file_type) {
                   available_sort_values.push(file_type) 
                }
            }
        },
        _ => {

        }
    }
    available_sort_values
}

pub async fn create_dir_by_sort_values(directory: &String, years: &Vec<String>) {
    for year in years {
        let folder_dir = format!("{}/{}", directory, year);
        match fs::create_dir(folder_dir.clone()){
            Ok(_) => print!("Success! Sort values for folders have been created!"),
            Err(e) => print!("{}", e)
        }
    }
}

// Copy to the files into the folders
pub async fn copy_files_into_folders(directory: &String, files: &Vec<FileInfo>, sort_type: &str){
    for file in files{
        let old_folder_dir = Path::new(directory).join(&file.name);
        let new_folder_dir = Path::new(directory);
        let target_folder_dir =  match sort_type {
            "year" => new_folder_dir.join(&file.year).join(&file.name),
            "filetype" => new_folder_dir.join(&file.file_type.as_str().to_string()).join(&file.name),
            _ => new_folder_dir.join(&file.year).join(&file.name)
        };
        let _ = fs::copy(old_folder_dir, target_folder_dir);
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