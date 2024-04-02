use std::{env, fs::File, io::Read, ops::BitAnd, process::exit, usize};

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

const CLASS_ACCESS_FLAGS: [(&str, u16); 8] = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_FINAL", 0x0010),
    ("ACC_SUPER", 0x0020),
    ("ACC_INTERFACE", 0x0200),
    ("ACC_ABSTRACT", 0x0400),
    ("ACC_SYNTHETIC", 0x1000),
    ("ACC_ANNOTATION", 0x2000),
    ("ACC_ENUM", 0x4000),
];

const METHOD_ACCESS_FLAGS: [(&str, u16); 12] = [
    ("ACC_PUBLIC", 0x0001),
    ("ACC_PRIVATE", 0x0002),
    ("ACC_PROTECTED", 0x0004),
    ("ACC_STATIC", 0x0008),
    ("ACC_FINAL", 0x0010),
    ("ACC_SYNCHRONIZED", 0x0020),
    ("ACC_BRIDGE", 0x0040),
    ("ACC_VARARGS", 0x0080),
    ("ACC_NATIVE", 0x0100),
    ("ACC_ABSTRACT", 0x0400),
    ("ACC_STRICT", 0x0800),
    ("ACC_SYNTHETIC", 0x1000),
];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: main <path_to_class_file>");
        exit(1);
    }

    let file_path = &args[1];
    let content = read_file2bytes(file_path);
    let mut parser = JavaClassFileParser::new(content);
    parser.parse();
}

#[derive(Debug)]
struct ByteStream {
    xs: Vec<u8>,
}

#[derive(Debug)]
struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

#[derive(Debug)]
enum Attr {
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        attrs: Vec<Attr>,
    },
    LineNumberTable {
        table: Vec<LineNumberTableEntry>,
    },
    SourceFile {
        file: String,
    },
}

#[derive(Debug)]
struct Method {
    access_flags: Vec<String>,
    name: String,
    descriptor: String,
    attrs: Vec<Attr>,
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
    count: usize,
    info: Vec<ConstantPoolInfo>,
}

impl ConstantPool {
    pub fn query(&self, index: usize) -> String {
        let index = index - 1;

        if index < 0 || index > self.count {
            eprintln!(
                "[ERROR]:{}:{}: Constant Pool Index out of bound",
                file!(),
                line!()
            );
            exit(1);
        }

        if self.info[index].tag != CONSTANT_UTF8 {
            return self.query(self.info[index].entries[0].1 as usize);
        }

        self.info[index].bytes.clone().unwrap()
    }
}

#[derive(Default)]
struct JavaClassFile {
    magic: u32,
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: Vec<String>,
    this_class: String,
    super_class: String,
    methods: Vec<Method>,
    attrs: Vec<Attr>,
}

impl JavaClassFile {
    pub fn dump(&self) {
        println!("Magic: 0x{:x}", self.magic);
        println!("Minor Version: 0x{:x}", self.minor);
        println!("Major Version: 0x{:x}", self.major);
        println!("Access Flags: {:?}", self.access_flags);
        println!(
            "This Class : {:?}\nSuper Class: {:?}",
            self.this_class, self.super_class
        );
        println!("{:#?}", self.methods);
        println!("{:#?}", self.attrs);
    }
}

struct JavaClassFileParser {
    bytes: ByteStream,
    class: JavaClassFile,
}

impl JavaClassFileParser {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: ByteStream { xs: bytes },
            class: JavaClassFile::default(),
        }
    }

    pub fn parse(&mut self) {
        self.class.magic = self.bytes.parse_u4();
        self.class.minor = self.bytes.parse_u2();
        self.class.major = self.bytes.parse_u2();
        self.class.constant_pool = self.parse_constant_pool();
        let access_flag_mask = self.bytes.parse_u2();
        self.class.access_flags = self.parse_access_flags(access_flag_mask, &CLASS_ACCESS_FLAGS);

        self.class.this_class = self
            .class
            .constant_pool
            .query(self.bytes.parse_u2() as usize);
        self.class.super_class = self
            .class
            .constant_pool
            .query(self.bytes.parse_u2() as usize);

        let interfaces_count = self.bytes.parse_u2();
        for _ in 0..interfaces_count {
            assert!(
                false,
                "[ERROR]:{}:{}: Interface parser not implemented yet\n",
                file!(),
                line!()
            );
        }

        let fields_count = self.bytes.parse_u2();
        for _ in 0..fields_count {
            assert!(
                false,
                "[ERROR]:{}:{}: Field parser not implemented yet\n",
                file!(),
                line!()
            );
        }

        self.class.methods = self.parse_methods();
        self.class.attrs = self.parse_attrs();

        self.class.dump();

        println!("Unparsed Class file content: {:?}", self.bytes.xs);
    }

    fn parse_methods(&mut self) -> Vec<Method> {
        let mut methods: Vec<Method> = vec![];

        for _ in 0..self.bytes.parse_u2() {
            let mask = self.bytes.parse_u2();
            let name_index = self.bytes.parse_u2();
            let descriptor_index = self.bytes.parse_u2();

            methods.push(Method {
                access_flags: self.parse_access_flags(mask, &METHOD_ACCESS_FLAGS),
                name: self.class.constant_pool.query(name_index as usize),
                descriptor: self.class.constant_pool.query(descriptor_index as usize),
                attrs: self.parse_attrs(),
            });
        }
        methods
    }

    fn parse_attrs(&mut self) -> Vec<Attr> {
        Self::__parse_attrs(&mut self.bytes, &self.class.constant_pool)
    }

    fn __parse_attrs(bytes: &mut ByteStream, cp: &ConstantPool) -> Vec<Attr> {
        let mut attrs: Vec<Attr> = vec![];

        for _ in 0..bytes.parse_u2() {
            let name = cp.query(bytes.parse_u2() as usize);
            let length = bytes.parse_u4();

            match name.as_ref() {
                "Code" => {
                    let mut code_attr_bytes = ByteStream {
                        xs: bytes.parse_n(length as usize),
                    };

                    let max_stack = code_attr_bytes.parse_u2();
                    let max_locals = code_attr_bytes.parse_u2();
                    let code_length = code_attr_bytes.parse_u4();
                    let code = code_attr_bytes.parse_n(code_length as usize);

                    for _ in 0..code_attr_bytes.parse_u2() {
                        assert!(
                            false,
                            "[ERROR]: exception_table parser not implemented yet!\n"
                        );
                    }

                    let nested_attrs = Self::__parse_attrs(&mut code_attr_bytes, cp);

                    attrs.push(Attr::Code {
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        attrs: nested_attrs,
                    });
                }
                "LineNumberTable" => {
                    let mut table: Vec<LineNumberTableEntry> = vec![];
                    let mut lnt_attr_bytes = ByteStream {
                        xs: bytes.parse_n(length as usize),
                    };
                    for _ in 0..lnt_attr_bytes.parse_u2() {
                        table.push(LineNumberTableEntry {
                            start_pc: lnt_attr_bytes.parse_u2(),
                            line_number: lnt_attr_bytes.parse_u2(),
                        })
                    }
                    attrs.push(Attr::LineNumberTable { table });
                }
                "SourceFile" => {
                    attrs.push(Attr::SourceFile {
                        file: cp.query(bytes.parse_u2() as usize) 
                    });
                }
                _ => {
                    eprintln!("[ERROR]:{}:{}: Unknown Attr: {name}", file!(), line!());
                    exit(1);
                }
            }
        }

        attrs
    }

    pub fn parse_access_flags(&self, mask: u16, access_flags: &[(&str, u16)]) -> Vec<String> {
        let mut flags: Vec<String> = vec![];
        for (name, value) in access_flags {
            if mask.bitand(value) != 0 {
                flags.push(name.to_string());
            }
        }
        flags
    }

    pub fn parse_constant_pool(&mut self) -> ConstantPool {
        let count = self.bytes.parse_u2();

        let mut constant_pool = ConstantPool {
            count: count as usize,
            info: vec![],
        };

        for _ in 0..(count - 1) {
            let tag = self.bytes.parse_u1();
            match tag {
                CONSTANT_CLASS => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_CLASS".to_string(),
                    entries: vec![("name_index".to_string(), self.bytes.parse_u2())],
                    bytes: None,
                }),
                CONSTANT_METHODREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_METHODREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), self.bytes.parse_u2()),
                        ("name_and_type_index".to_string(), self.bytes.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_NAMEANDTYPE => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_NAMEANDTYPE".to_string(),
                    entries: vec![
                        ("name_index".to_string(), self.bytes.parse_u2()),
                        ("descriptor_index".to_string(), self.bytes.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_UTF8 => {
                    let length = self.bytes.parse_u2();
                    constant_pool.info.push(ConstantPoolInfo {
                        tag,
                        tag_name: "CONSTANT_UTF8".to_string(),
                        entries: vec![("length".to_string(), length)],
                        bytes: Some(
                            String::from_utf8(self.bytes.parse_n(length as usize)).unwrap(),
                        ),
                    })
                }
                CONSTANT_FIELDREF => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_FIELDREF".to_string(),
                    entries: vec![
                        ("class_index".to_string(), self.bytes.parse_u2()),
                        ("name_and_type_index".to_string(), self.bytes.parse_u2()),
                    ],
                    bytes: None,
                }),
                CONSTANT_STRING => constant_pool.info.push(ConstantPoolInfo {
                    tag,
                    tag_name: "CONSTANT_STRING".to_string(),
                    entries: vec![("string_index".to_string(), self.bytes.parse_u2())],
                    bytes: None,
                }),
                _ => {
                    eprintln!("[ERROR]:{}:{}: Unknown tag {}", file!(), line!(), tag);
                    exit(1);
                }
            }
        }

        constant_pool
    }
}

impl ByteStream {
    fn parse_u1(&mut self) -> u8 {
        self.check_bound(1);
        self.xs.remove(0)
    }

    fn parse_u2(&mut self) -> u16 {
        self.check_bound(2);
        let res = self.xs[0..2].to_vec();
        self.xs = self.xs[2..].to_vec();
        u16::from_be_bytes(res.try_into().unwrap())
    }

    fn parse_u4(&mut self) -> u32 {
        self.check_bound(4);
        let res = self.xs[0..4].to_vec();
        self.xs = self.xs[4..].to_vec();
        u32::from_be_bytes(res.try_into().unwrap())
    }

    fn parse_n(&mut self, n: usize) -> Vec<u8> {
        self.check_bound(n);
        let res = self.xs[0..n].to_vec();
        self.xs = self.xs[n..].to_vec();
        res
    }

    fn check_bound(&self, n: usize) {
        if self.xs.len() < n {
            eprintln!("[ERROR]:{}:{}: Out of bound", file!(), line!());
            exit(1);
        }
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
