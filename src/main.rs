mod file_manager;

#[macro_use]
extern crate rocket;

use crate::file_manager::{FileId, FileQueryResponse};
use rocket::fs::TempFile;

use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

#[post("/upload", format = "image/png", data = "<file>")]
async fn upload_png(mut file: TempFile<'_>) -> std::io::Result<String> {
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

#[post("/upload", format = "image/jpeg", data = "<file>")]
async fn upload_jpeg(mut file: TempFile<'_>) -> std::io::Result<String> {
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

#[get("/download/<id>")]
async fn download(id: FileId<'_>) -> Option<File> {
    File::open(id.file_path("_low_cloaked.png")).await.ok()
}

#[get("/query/<id>")]
async fn query(id: FileId<'_>) -> Json<FileQueryResponse> {
    if id.file_path("_low_cloaked.png").exists() {
        Json(FileQueryResponse::READY)
    } else if id.file_path(".jpg").exists() {
        Json(FileQueryResponse::NotReady)
    } else {
        Json(FileQueryResponse::NotFound)
    }
}

#[launch]
fn rocket() -> _ {
    let _handler = thread::spawn(|| fawkes_runner());
    rocket::build().mount("/", routes![upload_png, upload_jpeg, download, query])
}

fn fawkes_runner() {
    println!("Thread spawned");
    let root = concat!(env!("CARGO_MANIFEST_DIR")).to_string();

    let program = &(root.clone() + "/protection").clone();
    let filepath = root.clone() + "/uploads";

    let mut command = Command::new(program);
    command.args(["-d", (&(root.clone() + "/uploads")), "-m", "low"]);

    loop {
        thread::sleep(Duration::from_secs(2));

        let _output = command.output().expect("Something went wrong");
        let paths = fs::read_dir(&filepath).expect("Couldn't find upload directory");

        for file in paths {

            let filename = file.as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .expect("Invalid file name");

            if filename.ends_with("_low_cloaked.png") {
                let file_age = file.unwrap().metadata().expect("Failed to get file metadata").created().expect("Failed to read files creation timestamp");
                if SystemTime::now().duration_since(file_age.clone()).expect("couldn't get system time") > Duration::from_secs(60*5){

                }

                match fs::remove_file(
                    filepath.clone() + "/" + &filename[0..filename.len() - 16] + ".png",
                ) {
                    Ok(_t) => {
                        println!("Processed file removed");
                        continue
                    }
                    Err(_e) => {
                        println!("Failed to remove processed file, this is a common error and can mostly be ignored")
                    }
                }
                match fs::remove_file(
                    filepath.clone() + "/" + &filename[0..filename.len() - 16] + ".jpg",
                ) {
                    Ok(_t) => {
                        println!("Processed file removed");
                        continue
                    }
                    Err(_e) => {
                        println!("Failed to remove processed file, this is a common error and can mostly be ignored")
                    }
                }
            }
        }
    }
}
