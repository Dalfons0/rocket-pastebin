#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rand;

mod paste_id;

use paste_id::PasteID;

use std::io;
use std::path::Path;
use std::fs::File;
use std::fs::write;

use rocket::Data;
use rocket::request::Form;
use rocket::response::content::Html;

fn main() {
    rocket::ignite().mount("/", routes![index, upload_data, upload_form, retrieve]).launch();
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html("
        <!DOCTYPE html>
        <html>
        <body>

        <h1>Rocket PasteBin</h1>

        <h2>USAGE</h2>

        <ul>
            <li>
            POST /

                accepts raw data in the body of the request and responds with a URL of
                a page containing the body's content
            </li>

            <li>
            GET /&#60;id&#62;

                retrieves the content for the paste with id `<id>`
            </li>
        </ul>

        
        <form method=\"post\" action=\"/\">
            <textarea name=\"content\" rows=\"30\" cols=\"80\"></textarea>
            <br>
            <input type=\"submit\" value=\"Paste!\">
        </form>
        </body>
        </html>
    ")
}

#[post("/", format = "text/plain", data = "<paste>")]
fn upload_data(paste: Data) -> io::Result<String> {
    let id = PasteID::new(10);
    let path = format!("upload/{}", id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&path))?;
    Ok(url)
}

#[derive(FromForm)]
struct Paste {
    content: String,
}

#[post("/", format = "application/x-www-form-urlencoded",data = "<paste>")]
fn upload_form(paste: Form<Paste>) -> io::Result<String> {
    let id = PasteID::new(10);
    let path = format!("upload/{}", id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    write(path,&paste.content)?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}