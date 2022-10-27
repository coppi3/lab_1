pub fn rz_code(inp: &[u8]) -> String {
    let mut out = String::new();
    let mut time = 0.0;

    for byte in inp {
        for i in (0..8).rev() {
            let code = |x:u8| {
                if (1 & x >> i)==1 {
                    1
                } else {
                    -1
                } }; 
            out = format!("{0}{1};{2}\n{3};{2}\n{3};{4}\n{5};{4}\n", out, time, code(*byte), time+0.5, 0, time + 1.0);
            time += 1.0;
        }
    }
    out
}
