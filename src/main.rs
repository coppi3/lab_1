mod utils;
use utils::utils::{encode, V};

use bitvec::prelude::*;
use std::fmt;
use std::fs;

fn main() {
    const INITMSG: &str = "Цыдыпов А.О.";
    let encoded_msg = encode(INITMSG);
    println!("Исходное сообщение: {}", INITMSG);
    println!("В шетснадцатеричном коде: {:X}", V(&encoded_msg));
    println!("В двоичном коде: {:08b}", V(&encoded_msg));
    println!(
        "Длина сообщения: {} байт ({} бит)",
        encoded_msg.len(),
        encoded_msg.len() * 8
    );
}
