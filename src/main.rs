use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use parser::parse;
use std::time::Instant;
use docopt::Docopt;
use std::process;

pub mod lexicon;
pub mod tokenizer;
pub mod parser;
pub mod grammar;
pub mod transformer;
pub mod codegen;
extern crate docopt;
extern crate rustc_serialize;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
honeybadger

Usage:
  badger [options]
  badger --version

Options:
  -h --help                 Show this screen.
  --version                 Show version.
  -f FILE, --file=FILE      Specifies the input file.
  -o FILE, --output=FILE    Specifies the output file.
  --ast                     Print out the Abstract Syntax Tree of the input.
";

fn read_file(path: &str) -> Result<String, Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err)
    }
}

fn write_file(filename: &str, program: String) -> Result<(), Error> {
    let mut f = try!(File::create(filename));
    match f.write_all(&program.into_bytes()[..]) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_file: Option<String>,
    flag_output: Option<String>,
    flag_version: bool,
    flag_ast: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("v{}", VERSION);
        process::exit(0);
    }

    if args.flag_file.is_none() {
        println!("{}", USAGE);
        process::exit(0);
    }

    let file = match read_file(&args.flag_file.unwrap()) {
        Ok(file) => file,
        Err(err) => {
            println!("ERR Couldn't read file: {:?}", err);
            process::exit(1)
        }
    };

    let ast = parser::parse(file);

    if args.flag_ast {
        println!("{:#?}", ast);
        process::exit(0);
    }

    let transformed_ast = transformer::traverse(ast);
    let program = codegen::generate_code(transformed_ast, false);

    if args.flag_output.is_none() {
        println!("{}", program);
        process::exit(0);
    }

    match write_file(&args.flag_output.unwrap(), program) {
        Ok(()) => {},
        Err(err) => {
            println!("ERR Writing out.js {}", err);
            process::exit(1);
        }
    }
}
