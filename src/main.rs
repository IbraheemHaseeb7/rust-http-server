use shuttle_actix_web::ShuttleActixWeb;
use actix_web::{get,  web,  HttpResponse};
use serde::ser::SerializeStruct;
use serde_json::to_string;
use core::panic;
use std::{fs, io::Error};

struct GetResponse {
    title: String,
    response: u16
}

impl serde::ser::Serialize for GetResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("GetResponse", 2)?;
            state.serialize_field("title", &self.title)?;
            state.serialize_field("response", &self.response)?;
            state.end()
        }
}

fn read_file(path: String) -> Result<String, Error> {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(_) => {
            return contents;
        }
        Err(_) => {
            panic!("Could not read file");
        }
    }
}

#[get("/")]
async fn hello() -> HttpResponse {
    let response: GetResponse = GetResponse {
        title: "Hello world".to_string(), response: 200
    };
    to_string(&response).unwrap();
    HttpResponse::Ok().json(response)
}

#[get("/site")]
async fn site() -> HttpResponse {
    let html_file: Result<String, Error> = read_file(String::from("./web/index.html"));
    let css_file: Result<String, Error> = read_file(String::from("./web/styles.css"));

    let final_html: String = generate_final_html_with_css(html_file.expect("Could not load HTML File"), css_file.expect("Could not load CSS File"));

    HttpResponse::Ok().body(final_html)
}

fn generate_final_html_with_css(html_file: String, css_file: String) -> String {
    let mut split_html_file = html_file.split("<!--We have to insert our CSS here using Rust Commands-->");

    let mut final_html:String = String::from(split_html_file.next().expect("Could not split!"));
    let css_file_string:String = css_file;

    final_html = final_html + "<style>" + &css_file_string + "</style>" + &String::from(split_html_file.next().expect("Could not split"));

    return final_html;
}


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(hello).service(site);
    };

    Ok(config.into())
}
