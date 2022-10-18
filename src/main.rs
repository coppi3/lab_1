mod utils;
use utils::utils::{encode, V};

use bitvec::prelude::*;
use std::fmt;
use std::fs;

fn main() {
    const init_msg: &str = "Цыдыпов А.О.";
    let encoded_msg = encode(init_msg);
    println!("{:X}", V(&encoded_msg));
}
