pub fn mlt3_code(inp: &[u8]) -> String {
    let mut out = String::new();
    let mut time = 0.0;
    // Reversing variable
    let mut rv = 1;
    let mut flagrv = 1;
    let mut prev = 0;

    for byte in inp {
        for i in (0..8).rev() {
            let mut code = |x: u8| {
                if (1 & x >> i) == 1 {
                    if rv == 1 {
                        rv = 0;
                        1
                    } else if rv == 0 {
                        if flagrv == 1 {
                            rv = -1;
                            flagrv = 0;
                        } else {
                            rv = 1;
                            flagrv = 1;
                        }
                        0
                    } else {
                        rv = 0;
                        -1
                    }
                } else {
                    prev
                }
            };
            prev = code(*byte);
            out = format!("{0}{1};{2}\n{3};{2}\n", out, time, prev, time + 1.0);
            time += 1.0;
        }
    }
    out
}
