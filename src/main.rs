#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rand;
extern crate rocket_contrib;

use rand::*;
use rocket::Data;
use rocket_contrib::serve::StaticFiles;
use std::char;
use std::io;
use std::path::Path;

fn gen_file_name() -> String {
  let mut file_name = String::new();
  for _ in 0..5 {
    let x: u32 = thread_rng().gen_range(0x1F600, 0x1F64F);
    let emoji = char::from_u32(x).unwrap();
    file_name.push(emoji);
  }
  return format!("{}.png", file_name);
}

#[get("/")]
fn index() -> &'static str {
  "Nothing to see here."
}

#[post("/upload", data = "<data>")]
fn upload(data: Data) -> io::Result<String> {
  let file_name = gen_file_name();
  let path = Path::new("static").join(&file_name);
  data
    .stream_to_file(path)
    .map(|_| format!("{}", file_name))
}

fn main() {
  rocket::ignite()
    .mount("/files", StaticFiles::from("static"))
    .mount("/", routes![upload, index])
    .launch();
}
