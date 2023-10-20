use rocket::Request;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    if let Ok(indexes) = std::fs::read_to_string("indexes") {
        return indexes;
    }
    String::from("UKE")
}

fn get_string(file: &mut File, start: usize, seek: usize) -> String {
    match file.seek(SeekFrom::Start(((start + 8) * 4).try_into().unwrap())) {
        Ok(_) => (),
        Err(_) => {return String::from("UKE");},
    };
    let mut tmp = vec![0u8; seek * 4];
    let _len = match file.read(&mut tmp) {
        Ok(d) => d,
        Err(_) => {return String::from("UKE");},
    };
    let mut tmp_start = 0;
    let mut p: Vec<char> = Vec::new();
    loop {
        if tmp_start + 4 > tmp.len() {break; }
        let gg: [u8; 4] = [tmp[tmp_start], tmp[tmp_start + 1], tmp[tmp_start + 2], tmp[tmp_start + 3]];
        p.push(char::from_u32(u32::from_be_bytes(gg)).unwrap());
        tmp_start += 4;
    }
    p.iter().collect::<String>()
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
        Err(_) => {return String::from("404");},
    };

    let mut buffer = [0u8; 8];
    let _n = file.read(&mut buffer[..]);
    let size: usize = usize::from_be_bytes(buffer);

    // Later +8 is added the size header.
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
    rocket::build().mount("/", routes![search_book]).mount("/", routes![index])
    .register("/", catchers![not_found])
    .register("/", catchers![bad_gateway])
}