pub mod lib {
    use mongodb::{
        bson::{self, bson, doc, from_bson, oid::ObjectId, Document},
        options::{ClientOptions, FindOptions},
        Client, Collection,
    };
    use rocket::{
        futures::TryStreamExt,
        serde::{
            json::{serde_json::json, Json, Value},
            Deserialize, Serialize,
        },
    };
    use std::env;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Doc {
        #[serde(rename = "_id")]
        pub id: ObjectId,
        pub item: String,
        pub price: f64,
        pub quantity: i32,
        pub date: bson::DateTime,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct JsonBody<'r> {
        item: &'r str,
        price: f64,
        quantity: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct UpdateBody<'r> {
        item: &'r str,
    }

    pub struct Options {
        pub filter_doc: Option<Document>,
        pub sort_option: Option<Document>,
    }
    impl Options {
        pub fn new(filter_doc: Option<Document>, sort_option: Option<Document>) -> Self {
            Self {
                filter_doc,
                sort_option,
            }
        }
    }

    pub struct Db;
    impl Db {
        pub async fn get_db() -> Mongodb {
            let coll = match Mongodb::connect().await {
                Ok(coll) => coll,
                Err(e) => panic!("{}", e),
            };
            let db = Mongodb { coll };
            db
        }
    }

    pub struct Mongodb {
        pub coll: mongodb::Collection<Document>,
    }
    impl Mongodb {
        pub fn get_env() -> (String, String, String) {
            let env_vars = env::vars();
            let mut uri = String::new();
            let mut db = String::new();
            let mut coll = String::new();

            for (k, v) in env_vars {
                match k.as_str() {
                    "URI" => uri = v,
                    "DB" => db = v,
                    "COLL" => coll = v,
                    _ => continue,
                }
            }
            assert!(!(uri.is_empty()));
            assert!(!(db.is_empty()));
            assert!(!(coll.is_empty()));

            (uri, db, coll)
        }

        pub async fn connect() -> mongodb::error::Result<Collection<Document>> {
            let (url, db, coll) = Self::get_env();

            let mut client_options = ClientOptions::parse(url).await?;
            client_options.app_name = Some("CRUD_EXAMPLE".to_string());

            let client = Client::with_options(client_options)?;
            let db = client.database(db.as_str());
            let coll: Collection<Document> = db.collection(coll.as_str());

            Ok(coll)
        }

        pub async fn get(
            &self,
            filter_doc: Option<Document>,
            sort_option: Option<Document>,
        ) -> Option<Json<Vec<Doc>>> {
            let coll = &self.coll;
            let mut documents: Vec<Doc> = Vec::new();

            let filter_option = FindOptions::builder().sort(sort_option).build();

            let mut cursor = coll
                .find(filter_doc.unwrap_or_default(), filter_option)
                .await
                .unwrap_or_else(|err| panic!("{}", err));
            {
                while let Ok(Some(d)) = cursor.try_next().await {
                    documents.push(from_bson(bson!(d)).unwrap());
                }
            }

            Some(Json(documents))
        }

        pub async fn post(&self, body: Json<JsonBody<'_>>) -> Value {
            let coll = &self.coll;
            let d = doc! {
                "item" : body.0.item,
                "price" : body.0.price,
                "quantity" : body.0.quantity,
                "date": bson::DateTime::now(),
            };

            coll.insert_one(&d, None)
                .await
                .unwrap_or_else(|err| panic!("{}", err));

            json!({
                "data" : d,
                "status_code": 200,
                "message": "success"
            })
        }

        pub async fn check_user(coll: &Collection<Document>, body: &Json<UpdateBody<'_>>) -> bool {
            let exist = coll
                .find_one(doc! {"item": body.0.item}, None)
                .await
                .unwrap();

            if exist.is_some() {
                true
            } else {
                false
            }
        }

        pub async fn update(&self, body: Json<UpdateBody<'_>>) -> Value {
            let coll = &self.coll;
            let exist = Self::check_user(coll, &body).await;

            if exist {
                let update = doc! {
                    "$set" : {
                        "item" : "new item",
                    }
                };
                coll.update_one(doc! {"item": body.0.item }, update, None)
                    .await
                    .unwrap_or_else(|err| panic!("failed to update :{}", err));
                json!({
                    "status_code" : 201,
                    "message" : "updated"
                })
            } else {
                json!({
                    "status_code" : 401,
                    "message" : "not found user"
                })
            }
        }

        pub async fn delete(&self, body: Json<UpdateBody<'_>>) -> Value {
            let coll = &self.coll;
            let exist = Self::check_user(coll, &body).await;

            if exist {
                coll.delete_one(doc! {"item" : body.0.item}, None)
                    .await
                    .unwrap_or_else(|err| panic!("{}", err));
                json!({
                    "status_code" : 200,
                    "message" : "deleted"
                })
            } else {
                json!({
                    "status_code" : 401,
                    "message" : "not found user"
                })
            }
        }
    }
}
