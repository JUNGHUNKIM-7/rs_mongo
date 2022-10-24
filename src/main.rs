use dotenvy::dotenv;
use mongodb::bson::doc;
use roc::lib::{Db, Doc, JsonBody, Options, UpdateBody};
use rocket::serde::json::{Json, Value};
use std::error::Error;

#[macro_use]
extern crate rocket;

#[get("/hello")]
async fn hello() -> Option<Json<Vec<Doc>>> {
    let ops = Options::new(None, Some(doc! {"id" : 1}));
    let db = Db::get_db().await;
    db.get(ops.filter_doc, ops.sort_option).await
}

#[post("/hello", format = "json", data = "<body>")]
async fn hello_post(body: Json<JsonBody<'_>>) -> Value {
    let db = Db::get_db().await;
    db.post(body).await
}

#[put("/hello", format = "json", data = "<body>")]
async fn hello_update(body: Json<UpdateBody<'_>>) -> Value {
    let db = Db::get_db().await;
    //pass to item name
    //find, then reutrn
    db.update(body).await
}

#[delete("/hello", format = "json", data = "<body>")]
async fn hello_delete(body: Json<UpdateBody<'_>>) -> Value {
    let db = Db::get_db().await;
    db.delete(body).await
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    const ROOT: &str = "/api";

    let _rocket = rocket::build()
        .mount(ROOT, routes![hello, hello_post, hello_update, hello_delete])
        .launch()
        .await?;
    Ok(())
}
