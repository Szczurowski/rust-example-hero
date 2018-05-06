#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

#[macro_use]
extern crate lazy_static;

use bson::Bson;
use mongodb::{Client, ClientOptions, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::connstring::{ConnectionString, ConnectionOptions, Host};
use mongodb::topology::{TopologyDescription, TopologyType};
use mongodb::stream::{StreamConnector};

use rocket_contrib::{Json, Value};

mod hero;
use hero::{Hero};

lazy_static! {
    static ref CLIENT: Client = {
        let client = Client::connect("cluster0-wqqy9.mongodb.net", 27017).unwrap();
        client.db("julas-db0").auth("mongol", "JFXKdHyaMNhluajT").unwrap();
        // client.db("julas-db0");
        client
    };
}

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    //let coll = get_db().collection("prescriptions");
    let coll = CLIENT.clone().db("julas-db0").collection("prescriptions");

    let doc = doc! { "title" => "Jaws",
                      "array" => [ 1, 2, 3 ] };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    hero
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "Natalka", 
        "Gabrysia",
        "Kasiusia",
        "Julasek"
    ]))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}

fn get_collection() -> Database {
    let client = Client::connect("localhost", 27017)
       .expect("Failed to initialize standalone client.");    
    
    client.db("julas-db0")
    // db.auth("mongol", "JFXKdHyaMNhluajT").unwrap();
}

/*let user = "mongol";
    let password = "JFXKdHyaMNhluajT";
    let host = "cluster0-wqqy9.mongodb.net".to_string();
    let db = "julas-db0";
    let collection = "prescriptions";

    let hosts = vec![Host{
        host_name: host,
        ipc: String::new(),
        port: 27017
    }];

    let mut client_options = ClientOptions::new();
    client_options.log_file = Some("/tmp/mongo-commands".to_string());

    let mut description = TopologyDescription::new(StreamConnector::Tcp);
    description.topology_type = TopologyType::Single;

    let connection_config = ConnectionString{
        hosts: hosts,
        string: None,
        user: Some(user.to_string()),
        password: Some(password.to_string()),
        database: None,
        collection: None,
        options: None
    };

    let client = Client::with_config(
        connection_config,
        Some(client_options),
        Some(description)
    ).ok().expect("Couldn't connect to mongodb database");

    let coll = client.db("julas-db0").collection("prescriptions");*/