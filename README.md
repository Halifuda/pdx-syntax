# PDX Syntax

This is a crate that defines the syntax of Paradox game files. It can be used to e
the plain text files into an AST it defines. The main purpose of this crate is to ide
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
    println!("{:#?}", ast);
}
```

## Supported Syntax

- Script: Paradox script. You can find the files in like `${game_root_dir}/game/common`.
Most of the files are in this format.
- Localization: Paradox localization file. You can find the files in `$e_root_dir}/game/localization`.

## Serialization

The AST defined in this crate can be serialized via [`serde`]. You can use this ure
to convert the AST into other formats, like JSON, and use the parsing results in r
languages, like Python.
