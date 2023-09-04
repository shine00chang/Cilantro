use super::*;

fn make (v: Vec<&'static str>) -> String {
    v.into_iter().fold(String::new(), |mut s, line| {
        s.push_str(line);
        s.push('\n');
        s
    })
}

impl Prog {
    pub fn lib () -> String {
        /*
        Code {
            main: "".to_owned(),
            global:
                "\
                ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written\n\
                (import \"wasi_unstable\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))\n\
                ;; first 20 bytes are reserved for printer\n\
                (memory 1)\n\
                (export \"memory\" (memory 0))\n\
                ".to_owned(),

            funcs: vec![
                "\n\
                (func $print_int (param $x i32)\n\
                    (i32.store (i32.const 8)\n\
                        (i32.add\n\
                            (i32.const 48)\n\
                            (i32.const 1)\n\
                        )\n\
                    )\n\
                    (i32.store (i32.const 9)\n\
                        (i32.add\n\
                            (i32.const 48)\n\
                            (i32.const 7)\n\
                        )\n\
                    )\n\
                    (i32.store (i32.const 0) (i32.const 8))\n\
                    (i32.store (i32.const 4) (i32.const 2))\n\
                    (call $fd_write\n\
                        (i32.const 1)\n\
                        (i32.const 0)\n\
                        (i32.const 1)\n\
                        (i32.const 16)\n\
                    )\n\
                    drop\n\
                )\n\
                ".to_owned(),
            ]
        }*/
        make(vec![
        // STDIO
        "  ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written",
        "  (import \"wasi_unstable\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))",

        // PRINT-INT
        "  ;; first 20 bytes are reserved for printer",
        "  (memory 1)",
        "  (export \"memory\" (memory 0))",
        "  (func $print_int (param $x i32)",
        "    (i32.store (i32.const 8)",
        "        (i32.add",
        "            (i32.const 48)",
        "            (i32.const 1)",
        "        )",
        "    )",
        "    (i32.store (i32.const 9)",
        "        (i32.add",
        "            (i32.const 48)",
        "            (i32.const 7)",
        "        )",
        "    )",
        "    (i32.store (i32.const 0) (i32.const 8))",
        "    (i32.store (i32.const 4) (i32.const 2))",
        "    (call $fd_write",
        "        (i32.const 1)",
        "        (i32.const 0)",
        "        (i32.const 1)",
        "        (i32.const 16)",
        "    )",
        "    drop",
        "  )"])
    }
}
