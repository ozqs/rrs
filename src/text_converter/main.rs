use std::{fs::File, io::Write};

#[allow(unused)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        fatal("参数太少！");
    }

    let input = &args[1];

    let content = std::fs::read_to_string(input)
        .unwrap_or_else(|e| fatal("打不开文件！"));

    let content: Vec<char> = content.chars().collect();

    let output_path = format!("{}.rstxt", input);

    let mut f = File::create(output_path.as_str())
        .unwrap_or_else(|e| fatal("打不开将写入的文件！"));

    f.write_all(&(content.len()).to_be_bytes());

    for ch in content {
        let uch = ch as u32;
        f.write_all(&uch.to_be_bytes());
    }
}

/// # fatal
/// 提供一个更加友好的 panic (?)
fn fatal(s: &str) -> ! {
    println!("Fatal: {s}");
    std::process::exit(1)
}
