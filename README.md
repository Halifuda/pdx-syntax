# PDX Syntax

This is a crate that defines the syntax of Paradox game files. It can be used to parse
the plain text files into an AST it defines. The main purpose of this crate is to provide
a convenient way to parse the files, which can be used for further tools, particularly,
tools for modders.

## Usage

```rust
/// Parse a given paradox script file.
use pdx_syntax::script::parse_file;
fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Missing argument: path");
    let ast = parse_file(&path).unwrap();
    // Do something with the AST.
    ast.push(Unit::SingleValue(Value::Primitive(Entry::Ident(
        "TEST".to_string(),
    ))));
    // Dump to file in origin syntax.
    println!("{}", ast);
}
```

## Supported Syntax

- Script: Paradox script. You can find the files in paths like `${game_root_dir}/game/common`.
Most of the files are in this format. See [`crate::script`] for details.
- Localization: Paradox localization file. You can find the files in `${game_root_dir}/game/localization`.
See [`crate::localization`] for details.

## Serialization

The AST defined in this crate can be serialized via [`serde`]. You can use this ure
to convert the AST into other formats, like JSON, and use the parsed results in other
languages, like Python.
