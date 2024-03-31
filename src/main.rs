use std::{borrow::Borrow, fs::File, io::Read, process::exit};

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_INTERFACEMETHODREF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAMEANDTYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHODHANDLE: u8 = 15;
const CONSTANT_METHODTYPE: u8 = 16;
const CONSTANT_INVOKEDYNAMIC: u8 = 18;

fn main() {
    let file_path = "samples/Main.class";
    let content = read_file2bytes(file_path);
    let mut parser = ParseClass::new(&content);
    parser.parse();
}

#[derive(Debug)]
struct ConstantPoolInfo {
    tag: u8,
    tag_name: String,
    entries: Vec<(String, u16)>,
    bytes: Option<String>,
}

#[derive(Default, Debug)]
struct ConstantPool {
    info: Vec<ConstantPoolInfo>,
}

#[derive(Default)]
struct JavaClassFile {
    magic: u32,
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
}

impl JavaClassFile {
    pub fn dump(&self) {
        println!("Magic: 0x{:x}", self.magic);
        println!("Minor Version: 0x{:x}", self.minor);
        println!("Major Version: 0x{:x}", self.major);
        println!("{:#?}", self.constant_pool);
    }
}

struct ParseClass<'a> {
    rawbytes: &'a [u8],
}

impl<'a> ParseClass<'a> {
    pub fn new(rawbytes: &'a [u8]) -> Self {
        Self { rawbytes }
    }

    pub fn parse(&mut self) {
        let mut class = JavaClassFile::default();
        class.magic = self.parse_u4();
        class.minor = self.parse_u2();
        class.major = self.parse_u2();
        class.constant_pool = self.parse_constant_pool();
        class.dump();
    }

    pub fn parse_constant_pool(&mut self) -> ConstantPool {
        let mut constant_pool = ConstantPool { info: vec![] };

        let count = self.parse_u2();
        for _ in 0..(count - 1) {
            let tag = self.parse_u1();
            match tag {
                CONSTANT_CLASS => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_CLASS".to_string(),
                    entries: vec![("name_index".to_string(), self.parse_u2())],
                    bytes: None,
                }),
                CONSTANT_METHODREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_METHODREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), self.parse_u2()),
                        ("name_and_type_index".to_string(), self.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_NAMEANDTYPE => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_NAMEANDTYPE".to_string(),
                    entries: vec![
                        ("name_index".to_string(), self.parse_u2()),
                        ("descriptor_index".to_string(), self.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_UTF8 => {
                    let length = self.parse_u2();
                    constant_pool.info.push(ConstantPoolInfo {
                        tag,
                        tag_name: "CONSTANT_UTF8".to_string(),
                        entries: vec![("length".to_string(), length)],
                        bytes: Some(
                            String::from_utf8(self.parse_n(length as usize).to_vec()).unwrap(),
                        ),
                    })
                }
                CONSTANT_FIELDREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_FIELDREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), self.parse_u2()),
                        ("name_and_type_index".to_string(), self.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_STRING => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_STRING".to_string(),
                    entries: vec![("string_index".to_string(), self.parse_u2())],
                    bytes: None
                }),
                _ => {
                    eprintln!("[ERROR]:{}:{}: Unknown tag {}", file!(), line!(), tag);
                    exit(1);
                }
            }
        }

        constant_pool
    }

    fn parse_u1(&mut self) -> u8 {
        if self.rawbytes.len() < 1 {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = self.rawbytes[0];
        self.rawbytes = &self.rawbytes[1..];
        res
    }

    fn parse_u2(&mut self) -> u16 {
        if self.rawbytes.len() < 2 {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = &self.rawbytes[0..2];
        self.rawbytes = &self.rawbytes[2..];
        u16::from_be_bytes(res.try_into().unwrap())
    }

    fn parse_u4(&mut self) -> u32 {
        if self.rawbytes.len() < 4 {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = &self.rawbytes[0..4];
        self.rawbytes = &self.rawbytes[4..];
        u32::from_be_bytes(res.try_into().unwrap())
    }

    fn parse_n(&mut self, n: usize) -> &[u8] {
        if self.rawbytes.len() < n {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
        let res = &self.rawbytes[0..n];
        self.rawbytes = &self.rawbytes[n..];
        res
    }
}

fn read_file2bytes(path: &str) -> Vec<u8> {
    let mut file = File::open(path)
        .map_err(|err| {
            eprintln!("[ERROR]: Couldn't open file: {err}");
        })
        .unwrap();

    let mut buf: Vec<u8> = vec![];
    File::read_to_end(&mut file, &mut buf)
        .map_err(|err| {
            eprintln!("[ERROR]: Couldn't read file content {err}");
        })
        .unwrap();

    buf
}
