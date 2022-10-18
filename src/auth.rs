use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Request, Rocket};
use rocket::figment::Source::File;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Serialize, Deserialize, json};
use serde_json::to_string;
use crate::file_manager::get_parent_path;
use crate::Status;

pub struct ApiKey<'a>(&'a str);
pub struct RootKey<'b>(&'b str);

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
}

#[rocket::async_trait]
impl Fairing for ValidKeys {
    fn info(&self) -> Info {
        Info {
            name: "ApiKey-Loader",
            kind: Kind::Liftoff | Kind::Shutdown
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        println!("liftoff");

        let key_storage = get_parent_path().to_str().unwrap().to_string()+"/keystore";
        let keymap = PathBuf::from(key_storage);

        if keymap.exists() {
            let mut keystore_file = fs::File::open(keymap).expect("Couldn't open keystore file");
            let mut keystore_content = &mut "".to_string();
            keystore_file.read_to_string(keystore_content).expect("Couldn't read keystore file to string");
            let _ = rocket.state().insert(&serde_json::from_str::<ValidKeys>(keystore_content));
        }else {
            let mut tmp_map = HashMap::new();
            tmp_map.insert("test".to_string(), 1);
            let _ = rocket::State();
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



#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey<'a> {
    type Error = KeyError;

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str, ) -> bool {
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

async fn is_valid() {

}

