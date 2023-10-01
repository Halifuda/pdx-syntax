use pdx_syntax::localization::parse_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Missing argument: path");
    let mut ast = parse_file(&path).unwrap();
    ast.entries.push(pdx_syntax::localization::types::Entry {
        key: "test".to_string(),
        num: Some(123),
        value: "\"test\"".to_string(),
    });
    println!("{}", ast);
}
