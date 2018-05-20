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
use mongodb::{Client, ThreadedClient};
use mongodb::db::{ThreadedDatabase};

use rocket_contrib::{Json, Value};

mod hero;
use hero::{Hero};

lazy_static! {
    static ref CLIENT: Client = {
        let client = Client::connect("localhost", 27017).unwrap();
        client.db("julas-db0");
        client
    };
}

// lazy_static! {
//     static ref CLIENT: Client = {
//         let client = Client::connect("cluster0-wqqy9.mongodb.net", 27017).unwrap();
//         client.db("julas-db0").auth("mongol", "JFXKdHyaMNhluajT").unwrap();
//         // client.db("julas-db0");
//         client
//     };
// }

// thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: OperationError("No servers available for the provided ReadPreference.")', libcore\result.rs:945:5
// note: Run with `RUST_BACKTRACE=1` for a backtrace.


#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    let coll = CLIENT.clone().db("julas-db0").collection("prescriptions");

    let doc = doc! { "title" => "Jaws",
                      "array" => [ 1, 2, 3 ] };

    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    hero
}

#[get("/")]
fn read() -> Json<Value> {
    let coll = CLIENT.clone().db("julas-db0").collection("prescriptions");
    let cursor = coll.find(None, None).ok().expect("Failed to read documents.");

    for result in cursor {
        // let doc = result.expect("Received network error during cursor operations.");
        // if let Some(&Bson::String(ref value)) = doc.get("spirit_animal") {
        //     println!("My spirit animal is {}", value);
        // }
        if let Ok(item) = result {
            println!("to_string {:?}", item.get_object_id("_id").unwrap().to_hex());

            match item.get_str("title") {
                Ok(value) => println!("title {:?}", value),
                Err(_e) => println!("title doesn't exist"),
            }
        }
    }

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
