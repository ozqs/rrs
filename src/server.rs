use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::process::exit;

pub fn get_rstxt_size(file: &mut File) -> usize {
    let mut buffer = [0u8; 8];

    match file.read(&mut buffer[..]) {
        Ok(_) => usize::from_be_bytes(buffer),
        Err(_) => 0,
    }
}

pub fn get_string(file: &mut File, start: usize, seek: usize) -> String {
    match file.seek(SeekFrom::Start(((start + 8) * 4).try_into().unwrap())) {
        Ok(_) => (),
        Err(_) => {
            return String::from("UKE");
        }
    };
    let mut tmp = vec![0u8; seek * 4];
    let _len = match file.read(&mut tmp) {
        Ok(d) => d,
        Err(_) => {
            return String::from("UKE");
        }
    };
    let mut tmp_start = 0;
    let mut p = Vec::new();

    loop {
        if tmp_start + 4 > tmp.len() {
            break;
        }
        let gg: [u8; 4] = [
            tmp[tmp_start],
            tmp[tmp_start + 1],
            tmp[tmp_start + 2],
            tmp[tmp_start + 3],
        ];
        p.push(char::from_u32(u32::from_be_bytes(gg)).unwrap());
        tmp_start += 4;
    }
    p.iter().collect::<String>()
}

fn phrase(args: Vec<String>) {
    match args[1].as_str() {
        "build" => {
            build();
        },
        _ => {
            usage();
            exit(1);
        }
    };
}

/// Show usage
fn usage() {
    println!("Usage: rrs [command]");
}

fn build() {
    if let Ok(mut file) = File::open("Config.conf") {
        let home_path = get_home_path(&mut file);
    }
}

fn get_home_path(file: &mut File) -> String {
    format!("Todo")
}