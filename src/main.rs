use std::env;
use std::process::exit;

mod parsers;
mod query;
mod utils;

use parsers::attributes::Attr;
use parsers::class::JavaClassFileParser;
use parsers::constantpool::ConstantPool;
use parsers::method::Method;
use query::query::Query;
use utils::bytestream::ByteStream;

use utils::prompt::prompt;
use utils::read_file_to_bytes::read_file_to_bytes;

#[derive(Default, Clone)]
pub struct JavaClassFile {
    pub magic: u32,
    pub minor: u16,
    pub major: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: Vec<String>,
    pub this_class: String,
    pub super_class: String,
    pub methods: Vec<Method>,
    pub attrs: Vec<Attr>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: main <path_to_class_file>");
        exit(1);
    }

    let file_path = &args[1];
    let content = read_file_to_bytes(file_path);
    let class_file = JavaClassFileParser::new(content).parse();
    let query = Query::new(&class_file);
    prompt(&class_file, query);
}
