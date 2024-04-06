use std::process::exit;

use super::{bytecode::parse_byte_code, constantpool::ConstantPool};
use crate::utils::bytestream::ByteStream;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Attr {
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<String>,
        attrs: Vec<Attr>,
    },
    LineNumberTable {
        table: Vec<LineNumberTableEntry>,
    },
    SourceFile {
        file: String,
    },
}

pub fn parse_attrs(bytes: &mut ByteStream, constantpool: &ConstantPool) -> Vec<Attr> {
    __parse_attrs(bytes, constantpool)
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
                let code = parse_byte_code(ByteStream {
                    xs: code_attr_bytes.parse_n(code_length as usize),
                }, cp);

                for _ in 0..code_attr_bytes.parse_u2() {
                    assert!(
                        false,
                        "[ERROR]: exception_table parser not implemented yet!\n"
                    );
                }

                let nested_attrs = __parse_attrs(&mut code_attr_bytes, cp);

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
                    file: cp.query(bytes.parse_u2() as usize),
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
