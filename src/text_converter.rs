use rrs::Fatal;
use rrs::MyString;
use std::fs::File;
use std::io::prelude::*;
use walkdir::WalkDir;

fn run(input: &str, output: &str) {
    let content = std::fs::read_to_string(input).fatal("打不开文件！");

    // let output_path = format!("{}.rstxt", input);

    let mut f = File::create(output).fatal("打不开将写入的文件！");

    f.write_all(&(content.len()).to_be_bytes())
        .fatal("写入文件时发生错误");

    // let content: Vec<char> = content.chars().collect();

    // for ch in content {
    //     let uch = ch as u32;
    //     f.write_all(&uch.to_be_bytes()).fatal("写入文件时发生错误");
    // }

    let mut conte = Vec::<u8>::new();
    content
        .chars()
        .for_each(|e| conte.append(&mut Vec::from((e as u32).to_be_bytes())));
    f.write_all(&conte).fatal("写入文件时发生错误");
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
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
