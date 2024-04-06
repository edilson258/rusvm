use super::constantpool::ConstantPool;
use crate::utils::bytestream::ByteStream;
use std::collections::HashMap;

const INVOKESPECIAL: u8 = 0xb7;
const GETSTATIC: u8 = 0xb2;
const LDC: u8 = 0x12;
const INVOKEVIRTUAL: u8 = 0xb6;

#[allow(unused)]
const BYTECODETABLE: [(u8, &str); 12] = [
    (
        0x1a,
        "iload_0         #load an int value from local variable 0",
    ),
    (
        0x1b,
        "iload_1         #load an int value from local variable 1",
    ),
    (0x60, "iadd            #add two ints"),
    (0x3d, "istore_2        #store int value into variable 2"),
    (
        0x1c,
        "iload_2         #load an int value from local variable 2",
    ),
    (0xac, "ireturn         #return an integer from a method"),
    (
        0x2a,
        "aload_0         #load a reference onto the stack from local variable 0",
    ),
    (
        0xb7,
        "invokespecial   #invoke instance method on object objectref",
    ),
    (0xb1, "return          #return void from method"),
    (0xb2, "getstatic       #get static field from class"),
    (
        0x12,
        "ldc             #push item from run-time constant pool",
    ),
    (0xb6, "invokevirtual   #invoke instance method"),
];

pub fn parse_byte_code(mut bytes: ByteStream, _: &ConstantPool) -> Vec<String> {
    let bytecodes: HashMap<u8, &str> = HashMap::from(BYTECODETABLE);

    let mut instructions: Vec<String> = vec![];
    while !bytes.xs.is_empty() {
        let x = bytes.parse_u1();

        if !bytecodes.contains_key(&x) {
            eprintln!("[ERROR]: Unknown Instruction 0x{:x}", x);
            continue;
        }

        match x {
            LDC => {
                let _ = bytes.parse_u1();
                let instr = bytecodes.get(&x).unwrap();
                instructions.push(instr.to_string())
            }
            INVOKESPECIAL | GETSTATIC | INVOKEVIRTUAL => {
                let _ = bytes.parse_u2();
                let instr = bytecodes.get(&x).unwrap();
                instructions.push(instr.to_string())
            }
            _ => {
                let instr = bytecodes.get(&x).unwrap();
                instructions.push(instr.to_string())
            }
        }
    }

    instructions
}
