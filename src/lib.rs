pub mod lib {
    use std::env;

    use mongodb::{
        bson::{bson, doc, from_bson, DateTime, Document},
        options::{ClientOptions, FindOptions},
        Client, Collection,
    };
    use rocket::futures::TryStreamExt;
    use rocket::serde::{json::Json, Deserialize, Serialize};

    pub enum Run {
        Get(DocumentArg),
        Post,
        Put,
        Delete,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct CollData {
        #[serde(rename = "_id")]
        pub id: i32,
        pub item: String,
        pub price: f64,
        pub quantity: i32,
        pub date: DateTime,
    }

    pub struct DocumentArg {
        filter_doc: Option<Document>,
        sort_option: Option<Document>,
    }

    pub struct CrudOps;

    pub struct Mongodb {
        pub coll: mongodb::Collection<Document>,
    }


    impl DocumentArg {
        pub fn new(filter_doc: Option<Document>, sort_option: Option<Document>) -> Self {
            Self {
                filter_doc,
                sort_option,
            }
        }
    }

    impl CrudOps {
        pub async fn run(run: Run) -> Option<Json<Vec<CollData>>> {
            let coll = match Mongodb::connect().await {
                Ok(coll) => coll,
                Err(e) => panic!("{}", e),
            };
            let db = Mongodb { coll };

            match run {
                Run::Get(doc_arg) => db.get(doc_arg.filter_doc, doc_arg.sort_option).await,
                _ => None,
            }
        }
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
        //GET COLLECTION
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
        ) -> Option<Json<Vec<CollData>>> {
            let coll = &self.coll;
            let mut documents: Vec<CollData> = Vec::new();

            let filter_option = FindOptions::builder().sort(sort_option).build();
            let mut cursor = match coll
                .find(filter_doc.unwrap_or_default(), filter_option)
                .await
            {
                Ok(val) => val,
                Err(_) => panic!(),
            };
            while let Ok(Some(d)) = cursor.try_next().await {
                documents.push(from_bson(bson!(d)).unwrap());
            }

            Some(Json(documents))
        }

        pub async fn post(data: CollData) {
            todo!()
        }

        pub async fn put(data: CollData) {
            todo!()
        }

        pub async fn delete(data: CollData) {
            todo!()
        }
    }
}
