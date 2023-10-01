use pdx_syntax::script::parse_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Missing argument: path");
    let mut ast = parse_file(&path).unwrap();
    use pdx_syntax::script::types::*;
    ast.push(Unit::SingleValue(Value::Primitive(Entry::Ident(
        "TEST".to_string(),
    ))));
    println!("{}", ast);
}
