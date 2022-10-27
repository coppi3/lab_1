mod utils;
use std::fmt;
use std::fs;
use utils::first::{encode, V};
use utils::manchester::manchester_code;
use utils::nrz::nrz_code;
use utils::rz::rz_code;
use utils::ami::ami_code;
use utils::mlt3::mlt3_code;
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

    // Second task:
    let manchester = manchester_code(&encoded_msg[0..4]);
    fs::write("../out/manchester.csv", manchester).expect("Can't write to manchester.csv");
    let nrz = nrz_code(&encoded_msg[0..4]);
    fs::write("../out/nrz.csv", nrz).expect("Can't write to nrz.csv");
    let rz = rz_code(&encoded_msg[0..4]);
    fs::write("../out/rz.csv", rz).expect("Can't write to rz.csv");
    let ami = ami_code(&encoded_msg[0..4]);
    fs::write("../out/ami.csv", ami).expect("Can't write to ami.csv");
    let mlt3 = mlt3_code(&encoded_msg[0..4]);
    fs::write("../out/mlt3.csv", mlt3).expect("Can't write to mlt3.csv");
}




/*
 * To do:
 * 1. Write all needed physical encodings
 * 2. Write logical encoding
 * 3. Write scrambling
 * 4. Refactor code to have encodings in utils folder.
 */
