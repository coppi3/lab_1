pub fn ami_code(inp: &[u8]) -> String {
    let mut out = String::new();
    let mut time = 0.0;
    // Reversing variable
    let mut rv = 1;
    for byte in inp {
        for i in (0..8).rev() {
            let mut code = |x: u8| {
                if (1 & x >> i) == 1 {
                    if rv == 1{
                        rv = 0;
                        1
                    }
                    else {
                        rv = 1;
                        -1
                    }
                } else {
                    0
                }
            };
            out = format!(
                "{0}{1};{2}\n{3};{2}\n",
                out,
                time,
                code(*byte),
                time + 1.0
            );
            time += 1.0;
        }
    }
    out
}
