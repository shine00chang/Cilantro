mod lib;
mod gen;

use super::*;

#[derive(Debug, Clone)]
struct Prog {
    global: Glob,
    funcs: Vec<Func>,
}
#[derive(Debug, Clone, Default)]
struct Glob {
    v: String,
    t: u32,
}
impl Glob {
    fn push (&mut self, s: String) {
        for _ in 0..self.t {
            self.v.push_str("  ");
        }
        self.v.push_str(s.as_str());
        self.v.push('\n');
        for c in s.chars() {
            match c {
                '(' => self.t += 1,
                ')' => self.t -= 1,
                _ => (),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Func {
    sig: String,
    p: String,
    v: String,
    t: u32
}
impl Func {
    fn new (sig: String) -> Self {
        Self {
            sig,
            p: String::new(),
            v: String::new(),
            t: 2
        }
    }
    fn push (&mut self, s: String) {
        let mut d: i32 = 0;
        for c in s.chars() {
            if c == '(' { d += 1 }
            if c == ')' { d -= 1 }
        }
        if d < 0 { self.t -= (-d) as u32 }
        for _ in 0..self.t {
            self.v.push_str("  ");
        }
        self.v.push_str(s.as_str());
        self.v.push('\n');
        if d > 0 { self.t += d as u32 }
    }
    fn prefix (&mut self, s: String) {
        self.p.push_str("    ");
        self.p.push_str(s.as_str());
        self.p.push('\n');
    }
    fn to_string (self) -> String {
        let mut code = String::new();
        code.push_str("  ");
        code.push_str(&self.sig);
        code.push('\n');
        code.push_str(&self.p);
        code.push('\n');
        code.push_str(&self.v);
        code.push_str("\n  )\n");

        code
    }
}



pub fn gen (nodes: Vec<LNode>) -> String {
    let mut prog = Prog { 
        global: Glob::default(),
        funcs: vec![]
    };
    let mut main = Func::new("(func $_main".to_owned());

    for node in nodes {
        node.codegen(&mut prog, &mut main);
    }

    to_string(prog, main)
}

fn to_string(prog: Prog, main: Func) -> String {
    let mut code = String::new();
    let lib = Prog::lib();

    code.push_str("(module\n");
    code.push_str(&lib.global.v);
    for f in lib.funcs {
        code.push_str(&f.to_string());
    }

    code.push_str(&prog.global.v);
    for f in prog.funcs {
        code.push_str(&f.to_string());
    }

    code.push_str(&main.to_string());

    code.push_str("  (export \"_start\" (func $_main))\n");
    code.push_str(")");
    code
}
