// Manchester encoding function
// So, basically this function returns a out String that has a csv-format that can then be saved into a
// file on the device.
//
// The format of each line in the out String is as follows:
// TIME;VALUE;
pub fn manchester_code(inp: &[u8]) -> String {
    let mut out = String::new();
    let mut time = 0;
    // let mut clock:u8 = 0;
    for byte in inp {
        for i in (0..8).rev() {
            for clock in 0..2 {
                out = format!("{2}{0};{1};\n", time, (clock ^ (1 & byte >> i)), out);
            }
            time += 1;
        }
    }
    out
}
