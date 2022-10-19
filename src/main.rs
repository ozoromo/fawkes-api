mod file_manager;
mod auth;
use auth::*;

#[macro_use]
extern crate rocket;
extern crate core;

use crate::file_manager::{FileId, FileQueryResponse, get_parent_path, KeyLoader};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs, thread};



//Currently using two paths for png and jpeg, hopefully this can be merged
/// Upload path for png files
#[post("/upload", format = "image/png", data = "<file>")]
async fn upload_png(mut file: TempFile<'_>, _key: ApiKey<'_>) -> std::io::Result<String> {
    println!("Got upload request");
    let id = FileId::new(10);
    dbg!(id.file_path(".jpg"));
    return match file.persist_to(&id.file_path(".jpg")).await {
        Ok(_t) => Ok(id.id.to_string()),
        Err(e) => {
            println!("Something went wrong");
            Err(e)
        }
    };
}

//TODO add a create key path

/// Upload path for jpeg files
#[post("/upload", format = "image/jpeg", data = "<file>")]
async fn upload_jpeg(mut file: TempFile<'_>, _key: ApiKey<'_>) -> std::io::Result<String> {
    println!("Got upload request");
    let id = FileId::new(10);
    return match file.persist_to(&id.file_path(".jpg")).await {
        Ok(_t) => Ok(id.id.to_string()),
        Err(e) => {
            println!("Something went wrong");
            Err(e)
        }
    };
}

/// Download path for getting images using their id
/// * `id` - The id used to select what image to download
#[get("/download/<id>")]
async fn download(id: FileId<'_>, _key: ApiKey<'_>) -> Option<File> {
    println!("Got download request");
    File::open(id.file_path("_low_cloaked.png")).await.ok()
}

/// Query path for getting a images status using its ID
#[get("/query/<id>")]
async fn query(id: FileId<'_>, _key: ApiKey<'_>) -> Json<FileQueryResponse> {
    println!("Got query request");
    if id.file_path("_low_cloaked.png").exists() {
        Json(FileQueryResponse::READY)
    } else if id.file_path(".jpg").exists() {
        Json(FileQueryResponse::NotReady)
    } else {
        Json(FileQueryResponse::NotFound)
    }
}

/// Basic health check path
/// Returns 200 when alive
#[get("/health")]
async fn health(_key: ApiKey<'_>) -> Status {
    Status::Ok
}

/// Function runs in separate thread and takes care of all things FAWKES
fn fawkes_runner() {
    println!("Thread spawned");
    let dir = get_parent_path();

    let root = dir
        .to_str()
        .expect("Couldn't parse executable path to string")
        .to_string();

    let program = &(root.clone() + "/protection").clone();
    let filepath = root.clone() + "/uploads";

    let mut command = Command::new(program);
    command.args(["-d", (&(root.clone() + "/uploads")), "-m", "low"]);

    loop {
        thread::sleep(Duration::from_secs(2));
        let _output = command.output().expect("Something went wrong");
        let paths = fs::read_dir(&filepath).expect("Couldn't find upload directory");

        for file in paths {
            let filename = &file
                .as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .expect("Invalid file name");

            if filename.ends_with("_low_cloaked.png") {
                let file_age = &file
                    .unwrap()
                    .metadata()
                    .expect("Failed to get file metadata")
                    .created()
                    .expect("Failed to read files creation timestamp");
                if SystemTime::now()
                    .duration_since(file_age.clone())
                    .expect("couldn't get system time")
                    > Duration::from_secs(60 * 5)
                {
                    match fs::remove_file(filepath.clone() + "/" + &filename) {
                        Ok(_) => {println!("Removed old file")}
                        Err(_) => {}
                    }
                }

                match fs::remove_file(
                    filepath.clone() + "/" + &filename[0..filename.len() - 16] + ".png",
                ) {
                    Ok(_t) => {
                        println!("Processed file removed");
                        continue;
                    }
                    Err(_e) => {}
                }
                match fs::remove_file(
                    filepath.clone() + "/" + &filename[0..filename.len() - 16] + ".jpg",
                ) {
                    Ok(_t) => {
                        println!("Processed file removed");
                        continue;
                    }
                    Err(_e) => {}
                }
            }
        }
    }
}

/// Builds and launches the rocket application.
/// This is also the replacement for the main function
#[launch]
fn rocket() -> _ {
    let _handler = thread::spawn(|| fawkes_runner());
    rocket::build().attach(KeyLoader).manage(ValidKeys::from_file()).mount(
        "/",
        routes![upload_png, upload_jpeg, download, query, health],
    )
}