pub fn nrz_code(inp: &[u8]) -> String {
    let mut out = String::new();
    let mut time = 0;

    for byte in inp {
        for i in (0..8).rev() {
            out = format!("{}{};{}\n{1};{2}\n", out, time, (1 & byte >> i));
            time += 1;
        }
    }
    out
}
