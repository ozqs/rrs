use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::process::exit;

pub fn fatal(s: &str) -> ! {
    println!("Fatal: {s}");
    std::process::exit(1)
}

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

pub trait Fatal<T, E: fmt::Display> {
    fn fatal(self, msg: &str) -> T;
}

impl<T, E: fmt::Display> Fatal<T, E> for Result<T, E> {
    fn fatal(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                println!("Fatal: {}\n错误信息: {}", msg, e.to_string());
                exit(1);
            }
        }
    }
}

pub fn run_for_text_converter(input: &str) {
    let content = std::fs::read_to_string(input).fatal("打不开文件！");

    let content: Vec<char> = content.chars().collect();

    let output_path = format!("{}.rstxt", input);

    let mut f = File::create(output_path.as_str()).fatal("打不开将写入的文件！");

    f.write_all(&(content.len()).to_be_bytes())
        .fatal("写入文件时发生错误");

    for ch in content {
        let uch = ch as u32;
        f.write_all(&uch.to_be_bytes()).fatal("写入文件时发生错误");
    }
}
