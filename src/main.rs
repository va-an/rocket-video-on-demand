use std::{env, path::PathBuf};

use rocket::{fs::NamedFile, http, response::status};

#[macro_use]
extern crate rocket;

#[get("/<path..>")]
pub async fn get_video_file(path: PathBuf) -> Result<NamedFile, status::Custom<String>> {
    let data_dir = env::var("DATA_DIR").expect("DATA_DIR must be set");
    let path = PathBuf::from(data_dir).join(path.clone());

    match NamedFile::open(path).await {
        Ok(file) => Ok(file),
        Err(e) => Err(status::Custom(http::Status::NotFound, e.to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_video_file])
}
