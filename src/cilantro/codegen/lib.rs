use super::*;

impl Prog {
    pub fn lib () -> Self {
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
        let global = Glob::default(); 
        let funcs = vec![];
        Self {
            global,
            funcs,
        }
    }
}
