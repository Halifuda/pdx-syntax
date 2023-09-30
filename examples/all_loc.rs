use pdx_syntax::localization::parse_file;
use std::path::Path;

fn parse_dir(dir: &Path) {
    let files = std::fs::read_dir(dir).unwrap();
    for file in files {
        let file = file.unwrap();
        let path = file.path();

        if path.is_dir() {
            parse_dir(&path);
            continue;
        }

        let fname = path.to_str().unwrap();
        if fname.ends_with(".yml") {
            // println!("[Parsing] {}", fname);
            let ast = parse_file(&fname);
            if ast.is_err() {
                println!("Parsing failed: {}", fname);
                println!("Error: {}", ast.err().unwrap());
            } else {
                // println!("{:#?}", ast.ok().unwrap());
            }
        }
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let fname = args.next().expect("Missing argument: path");
    let path = Path::new(&fname);
    if path.is_dir() {
        parse_dir(&path);
        return;
    } else {
        panic!("Not a directory: {}", fname);
    }
}
