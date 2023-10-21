mod server;
use rocket::Request;
use std::fs::File;
use server::*;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    if let Ok(indexes) = std::fs::read_to_string("indexes") {
        return indexes;
    }
    String::from("404")
}

#[get("/<name>/<start>/<seek>")]
fn search_book(name: &str, start: usize, seek: usize) -> String {
    let mut name = String::from(name);
    name.push_str(".txt.rstxt");
    if name.contains('/') || name.contains('\\') {
        return String::from("PSD"); // 你访问啥?! ()
    }

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
    rocket::build()
        .mount("/", routes![search_book])
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .register("/", catchers![bad_gateway])
}
