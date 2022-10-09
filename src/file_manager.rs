use rocket::serde::Serialize;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use rand::Rng;
use rocket::request::FromParam;


#[derive(Serialize)]
pub struct FileId<'a> {
    pub id: Cow<'a, str>
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
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads", "/").to_string() + self.id.as_ref() + extension;
        Path::new(&root).to_path_buf()
    }

}

impl<'a> FromParam<'a> for FileId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_ascii_alphanumeric())
            .then(|| FileId { id: param.into() })
            .ok_or(param)
    }
}
