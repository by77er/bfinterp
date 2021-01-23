mod bf;

use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout};

fn main() {
    let r = BufReader::new(File::open("bf/mandelbrot.bf").unwrap());

    let t = bf::Lexer::new(r);
    let p = bf::Parser::new(t);
    let mut c = bf::generate_code(p.collect());
    c.reverse();

    // println!("{:?}", c);
    // println!("total instructions: {:?}", c.len());

    let mut si = stdin();
    let mut so = stdout();

    let mut bf = bf::Interpreter::new(&c, &mut si, &mut so);
    bf.run().unwrap();
}
