use dotenvy::dotenv;
use mongodb::bson::doc;
use roc::lib::{CollData, CrudOps, DocumentArg, Run};
use rocket::serde::json::Json;
use std::error::Error;

#[macro_use]
extern crate rocket;

#[get("/hello")]
async fn hello() -> Option<Json<Vec<CollData>>> {
    let doc = DocumentArg::new(None, Some(doc! {"id" : 1}));
    CrudOps::run(Run::Get(doc)).await
}

#[get("/hello/<id>")]
fn hello_by_id(id: &str) -> String {
    todo!()
}

#[get("/hello/<id>/<name>/<cool>")]
fn hello_multi(id: &str, name: &str, cool: bool) -> String {
    todo!()
}

#[get("/hello/<_>/world")]
fn hello_world() -> &'static str {
    "hello/whatever/world"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    const ROOT: &str = "/api";

    let _rocket = rocket::build()
        .mount(ROOT, routes![hello, hello_by_id, hello_multi, hello_world])
        // .mount(ROOT, FileServer::from("static/"))
        .launch()
        .await?;
    Ok(())
}
