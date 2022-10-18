mod utils;
use std::fmt;
use std::fs;
use utils::utils::{encode, V};
const DEL: &str = "------------------------------------------------------";

fn main() {
    // First task:
    const INITMSG: &str = "Цыдыпов А.О.";
    let encoded_msg = encode(INITMSG);
    println!("{DEL}");
    println!("Исходное сообщение: {}", INITMSG);
    println!("В шетснадцатеричном коде: {:X}", V(&encoded_msg));
    println!("В двоичном коде: {:08b}", V(&encoded_msg));
    println!(
        "Длина сообщения: {} байт ({} бит)\n",
        encoded_msg.len(),
        encoded_msg.len() * 8
    );
    println!("{DEL}");
    let tmp = manchester_code(&encoded_msg);

    // Second task:
}

fn manchester_code(inp: &[u8]) -> String {
    let out = String::new();
    for byte in inp {
        for i in (0..8).rev() {
            if byte >> i & 1 == 1 {
                /*
                 * TO DO
                 */
                println!("{}", i);
            }
        }
    }
    out
}

/*
 * To do:
 * 1. Write all needed physical encodings
 * 2. Write logical encoding
 * 3. Write scrambling
 * 4. Refactor code to have encodings in utils folder.
 */
