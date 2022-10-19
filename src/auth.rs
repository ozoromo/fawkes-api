
use std::collections::HashMap;
use std::fs;
use std::io::{Read};
use std::path::PathBuf;
use rocket::{Request};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Serialize, Deserialize};
use crate::file_manager::get_parent_path;
use crate::{rocket, Status};

pub struct ApiKey<'a>(&'a str);
pub struct RootKey<'b>(&'b str);
//TODO load root key from env or similar

#[derive(Debug)]
pub enum KeyError {
    Missing,
    Invalid,
}

#[derive(Serialize, Deserialize)]
pub struct ValidKeys {
    pub(crate) keys: Option<HashMap<String, i32>>,
}

impl ValidKeys {
    pub fn new() -> ValidKeys {
        ValidKeys {keys: None}
    }

    pub fn from_file() -> ValidKeys {
        let key_storage = get_parent_path().to_str().unwrap().to_string()+"/keystore";
        let keymap = PathBuf::from(key_storage);

        return if keymap.exists() {
            let mut keystore_file = fs::File::open(keymap).expect("Couldn't open keystore file");
            let keystore_content = &mut "".to_string();
            keystore_file.read_to_string(keystore_content).expect("Couldn't read keystore file to string");
            serde_json::from_str::<ValidKeys>(keystore_content).expect("Couldn't parse keystore file")
        } else {
            ValidKeys::new()
        }
    }
}




#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey<'a> {
    type Error = KeyError;

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(_key: &str, ) -> bool {
            //TODO do key check
            unimplemented!()
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, KeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, KeyError::Invalid)),
        }
    }
}


#[rocket::async_trait]
impl<'b> FromRequest<'b> for RootKey<'b> {
    type Error = KeyError;

    async fn from_request(req: &'b Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, KeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(RootKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, KeyError::Invalid)),
        }
    }
}
