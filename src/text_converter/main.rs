use rrs::fatal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        fatal("参数太少！");
    }

    let input = &args[1];

    rrs::run_for_text_converter(&input);
}
