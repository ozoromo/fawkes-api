use rand::Rng;
use rocket::request::FromParam;
use rocket::serde::Serialize;
use std::borrow::Cow;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct FileId<'a> {
    pub id: Cow<'a, str>,
}

#[derive(Serialize)]
pub enum FileQueryResponse {
    NotReady,
    READY,
    NotFound,
}

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
        let exe_path = match env::current_exe() {
            Ok(exe_path) => exe_path,
            Err(_e) => panic!("Unable to get executable path"),
        };

        let dir = exe_path.parent().expect("Executable has no parent path");

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


pub fn get_parent_path() -> PathBuf {
    let exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(_e) => panic!("Unable to get executable path"),
    };

    let dir = exe_path.parent().expect("Executable has no parent path");
    return dir.clone().to_path_buf()
}
