use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use read_input::prelude::*;
use uuid::Uuid;
use lab01_2022_input_validation::*;

lazy_static! {
    // (String, bool) where the string is the filepath and the bool is true if a movie (is_movie)
    static ref HASHMAP: Mutex<HashMap<Uuid, (String, bool)>> = Mutex::new(HashMap::new());
    static ref NAMESPACE: Uuid = Uuid::parse_str("c7bb890c-a4a8-4d68-85b7-1e1cfe909249").unwrap();
}

fn file_upload_handler() {
    loop {
        let filepath = input::<String>().repeat_msg("Please enter the path to an image or video file : ").get();
        match validate_file(&filepath, true) {
            Ok(result) => match result {
                0 => println!("Invalid file contents !"),
                i => {
                    // Generate v5 uuid
                    let key = Uuid::new_v5(&NAMESPACE, filepath.to_lowercase().as_bytes());

                    // Check that the file is not already present => break if so
                    let mut map = HASHMAP.lock().unwrap();
                    if map.contains_key(&key) {
                        println!("This file is already uploaded.\n");
                        break;
                    } else {
                        // true for videos (2), false for images (1)
                        map.insert(key, (filepath, i == 2));
                        println!("File uploaded successfully, UUID : {}\n", key.to_string());
                        break;
                    }
                }
            },
            Err(e) => println!("{}", e.to_string()),
        }
    }
}

fn file_verify_handler() {
    loop {
        let uuid = input::<String>().repeat_msg("Please enter the UUID to check : ").get();
        if validate_uuid(&uuid) {
            let map = HASHMAP.lock().unwrap();

            match map.get(&Uuid::parse_str(&uuid).unwrap()) {
                None => println!("File {} doesn't exist.\n", uuid),
                Some((_, is_video)) => {
                    if *is_video {
                        println!("File {} exists, it is a video file.\n", uuid);
                    } else {
                        println!("File {} exists, it is an image file.\n", uuid);
                    }
                }
            }
            break;
        } else {
            println!("Invalid uuid !");
        }
    }
}

fn get_url_handler() {
    loop {
        let uuid = input::<String>().repeat_msg("Please enter the UUID to get : ").get();
        if validate_uuid(&uuid) {
            let map = HASHMAP.lock().unwrap();

            match map.get(&Uuid::parse_str(&uuid).unwrap()) {
                None => println!("File {} doesn't exist.\n", uuid),
                Some((filepath, is_video)) => {
                    // Generate url
                    if *is_video {
                        println!("sec.upload/videos/{}\n", filepath);
                    } else {
                        println!("sec.upload/images/{}\n", filepath);
                    }
                }
            }
            break;
        } else {
            println!("Invalid uuid !");
        }
    }
}

fn main() {
    println!("Welcome to the super secure file upload tool !");
    loop {
        match input::<i32>().repeat_msg("Please select one of the following options to continue :\n1 - Upload a file\n2 - Verify file exists\n3 - Get file URL\n0 - Exit\nYour input ? [0-3] ")
            .min_max(0, 3).get() {
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => file_upload_handler(),
            2 => file_verify_handler(),
            3 => get_url_handler(),
            _ => panic!("Invalid input"),
        }
    }
}
