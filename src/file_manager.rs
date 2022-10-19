use rand::Rng;
use rocket::request::FromParam;
use rocket::serde::Serialize;
use std::borrow::Cow;
use std::{env, fs};
use std::io::Write;
use std::path::{Path, PathBuf};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Rocket};
use crate::ValidKeys;

pub struct KeyLoader;

/// File id used in various queries
#[derive(Serialize)]
pub struct FileId<'a> {
    pub id: Cow<'a, str>,
}

/// Enum listing the possible responses when querying for a certain file
#[derive(Serialize)]
pub enum FileQueryResponse {
    NotReady,
    READY,
    NotFound,
}

///Implements Fairing for KeyLoader in order to automatically save keys to storage on shutdown
#[rocket::async_trait]
impl Fairing for KeyLoader {
    fn info(&self) -> Info {
        Info {
            name: "ApiKey-Loader",
            kind: Kind::Shutdown
        }
    }

    async fn on_shutdown(&self, rocket: &Rocket<Orbit>) {
        println!("on shutdown ran");

        let keystorage = get_parent_path().to_str().unwrap().to_string()+"/keystore";
        let keymap_file = PathBuf::from(keystorage);

        let keymap  = rocket.state::<ValidKeys>().expect("Couldn't access api key map");

        let mut file_handler = fs::File::create(keymap_file).expect("Couldn't create keystore file");
        file_handler.write(serde_json::to_string(keymap).expect("Couldn't serialize keystore").as_ref()).expect("Couldn't write to keystore file");
        file_handler.flush().expect("Couldn't write to keystore file");
    }
}

/// Utility functions for the fileID struct
impl FileId<'_> {
    pub fn new(size: usize) -> Self {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        FileId { id: Cow::Owned(id) }
    }

    pub fn file_path(&self, extension: &str) -> PathBuf {
       let dir = get_parent_path(); 

        let root = dir
            .to_str()
            .expect("Couldn't parse executable path to string")
            .to_string()
            + "/uploads/"
            + self.id.as_ref()
            + extension;
        Path::new(&root).to_path_buf()
    }
}

///Allows checking if fileID is correctly formatted in requests
impl<'a> FromParam<'a> for FileId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| FileId { id: param.into() })
            .ok_or(param)
    }
}

///Gets the executables parent directory path
pub fn get_parent_path() -> PathBuf {
    let exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(_e) => panic!("Unable to get executable path"),
    };

    let dir = exe_path.parent().expect("Executable has no parent path");
    return dir.clone().to_path_buf()
}
