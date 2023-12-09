use rrs::Fatal;
use rrs::MyString;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use walkdir::WalkDir;

/// ## To bytes
/// 将一个字符串的每个字符转换成四个字节后合并为一个 Vec<u8>
/// ## Example:
/// ```rust
/// let s = "Hello";
/// let d = to_bytes(s);
///
/// assert_eq!(d, vec![0u8, 0u8, 0u8, 104u8, 0u8, 0u8, 0u8, 101u8, 0u8, 0u8,
///  0u8, 108u8, 0u8, 0u8, 0u8, 108u8, 0u8, 0u8, 0u8, 111u8,])
/// ```


// fn to_bytes(input: &str) -> Vec<u8> {
//     let mut content = Vec::new();
//     input
//         .chars()
//         .for_each(|e| content.append(&mut Vec::from((e as u32).to_be_bytes())));

//     content
// }

/// ## run
/// 给定输入路径，输出路径，生成.rrs文件
fn run(input: &str, output: &str) {
    let content = std::fs::read_to_string(input).fatal("打不开文件！");

    let mut f = File::create(output).fatal("打不开将写入的文件！");

    f.write_all(&(content.len()).to_be_bytes())
        .fatal("写入文件时发生错误");

    let content: Vec<u8> = content
        .chars()
        .flat_map(|x| (x as u32).to_be_bytes())
        .collect();

    f.write_all(&content).fatal("写入文件时发生错误");
}

pub fn build() -> Result<(), Box<dyn Error>> {
    _ = std::fs::create_dir("target");
    let mut indexes = File::create("target/indexes")?;

    for entry in WalkDir::new("src")
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let input = entry.path().to_string_lossy().to_string();
        if !input.ends_with(".txt") {
            continue;
        }

        let output = input
            .replace_last("src", "target")
            .replace_last("txt", "rrs");

        print!("Processing {} to {} ...", input, output);
        indexes.write_all(
            entry
                .file_name()
                .to_string_lossy()
                .to_string()
                .replace_last(".txt", "\n")
                .as_bytes(),
        )?;
        run(&input, &output);
        println!("done.");
    }

    Ok(())
}
