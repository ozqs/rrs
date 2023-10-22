mod server;
mod text_converter;
mod usage;

use rocket::Request;
use server::*;
use std::{fs::File, process::exit};
use text_converter::*;
use usage::*;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    if let Ok(indexes) = std::fs::read_to_string("target/indexes") {
        return indexes;
    }
    String::from("404")
}

#[get("/<name>/<start>/<seek>")]
fn search_book(name: &str, start: usize, seek: usize) -> String {
    let mut name = String::from(name);
    name.push_str(".rrs");
    if name.contains('/') || name.contains('\\') {
        return String::from("PSD"); // 你访问啥?! ()
    }

    name = format!("target/{}", name);

    let mut file = match File::open(name) {
        Ok(f) => f,
        Err(_) => {
            return String::from("404");
        }
    };

    let size = get_rstxt_size(&mut file);

    // Later +8 is added the size header.
    // 判断边界
    if start + 8 > size {
        String::from("404")
    } else if start + seek + 8 > size {
        get_string(&mut file, start, size - start)
    } else {
        get_string(&mut file, start, seek)
    }
}

#[catch(404)]
fn not_found(_req: &Request) -> String {
    format!("404")
}

#[catch(502)]
fn bad_gateway(_req: &Request) -> String {
    format!("502")
}

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        exit(1);
    }

    if args[1] == "build" {
        exit(match build() {
            Ok(_) => 0,
            Err(e) => {
                println!("{}", e.to_string());
                1
            }
        });
    } else if args[1] == "run" {
        ()
    } else if args[1] == "help" {
        usage();
        exit(0);
    } else {
        println!("Unknown: {}\n", args[1]);
        usage();
        exit(1);
    }

    rocket::build()
        .mount("/", routes![search_book])
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .register("/", catchers![bad_gateway])
}
