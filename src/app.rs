
use rocket::{get, launch, routes, uri};
use rocket::response::Redirect;
use rocket::fs::NamedFile;
// use rocket_contrib::serve::StaticFiles;

use std::path::Path;
use std::io;

#[get("/")]
fn index() -> Redirect {
    return Redirect::to(uri!("/home"))
}

#[get("/home")]
async fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/home.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,home])
}

// #[macro_use] extern crate rocket;

// use rocket::fs::NamedFile;
// use std::path::{Path, PathBuf};

// #[get("/home")]
// async fn home() -> Option<NamedFile> {
//     NamedFile::open(Path::new("static/home.html")).await.ok()
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![home])
// }
