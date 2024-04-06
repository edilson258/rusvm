use std::{fs::File, io::Read};

pub fn read_file_to_bytes(path: &str) -> Vec<u8> {
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
