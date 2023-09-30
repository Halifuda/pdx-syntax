use pdx_syntax::script::parse_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Missing argument: path");
    let ast = parse_file(&path).unwrap();
    println!("{:#?}", ast);
}
