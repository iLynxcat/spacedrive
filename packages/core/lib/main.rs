use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs;

pub mod crypto;
pub mod db;
pub mod file;
pub mod native;
pub mod util;
use futures::executor::block_on;

// static configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub data_dir: std::path::PathBuf,
    pub primary_db: std::path::PathBuf,
    pub file_type_thumb_dir: std::path::PathBuf,
}

// represents an event this library can emit
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientEvent {
    NewFileTypeThumb,
}

// represents an application or client instance to communicate with
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalClient {
    pub config: AppConfig,
}

trait Emitter {
    fn emit(&self, event: ClientEvent, data: &str) -> Result<(), String>;
}

impl Emitter for ExternalClient {
    fn emit(&self, event: ClientEvent, _: &str) -> Result<(), String> {
        Ok(())
    }
}

pub static EXTERNAL_CLIENT: OnceCell<ExternalClient> = OnceCell::new();

pub fn configure(mut data_dir: std::path::PathBuf) {
    data_dir = data_dir.join("spacedrive");

    let config = AppConfig {
        data_dir: data_dir.clone(),
        primary_db: data_dir.clone().join("primary.db3"),
        file_type_thumb_dir: data_dir.clone().join("file_icons"),
    };

    EXTERNAL_CLIENT.set(ExternalClient { config });

    // EXTERNAL_CLIENT
    //     .unwrap()
    //     .emit(ClientEvent::NewFileTypeThumb, "...");

    // create the data directories if not present
    fs::create_dir_all(&EXTERNAL_CLIENT.get().unwrap().config.data_dir).unwrap();
    fs::create_dir_all(&EXTERNAL_CLIENT.get().unwrap().config.file_type_thumb_dir).unwrap();

    // create primary data base if not exists
    block_on(db::connection::create_primary_db()).unwrap();
    // init filesystem and create library if missing
    block_on(file::init::init_library()).unwrap();

    println!("Spacedrive daemon online");
}

// pub static MAIN_WINDOW: OnceCell<> = OnceCell::new();
// // handler to pass "callback" to OneCell intercepting the commands
// pub fn emit(kind: &str, data: &str) {
//     let _message = MAIN_WINDOW
//         .get()
//         .unwrap()
//         .emit(kind, data)
//         .map_err(|e| println!("{}", e));
// }

fn main() {
    // hello!
    println!("Hello, world!");
}
