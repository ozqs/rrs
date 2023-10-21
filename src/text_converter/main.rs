use rrs::fatal;
mod text_converter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        fatal("参数太少！");
    }

    let input = &args[1];

    text_converter::run(&input);
}
