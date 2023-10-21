use rrs::Fatal;
use std::fs::File;
use std::io::prelude::*;

pub fn run(input: &str) {
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
