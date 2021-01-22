mod bf;

use std::io::{stdin, stdout};
use std::io::BufReader;
use std::fs::File;

fn main() {
    let r = BufReader::new(File::open("test.bf").unwrap());
    let i = bf::token::Tokenizer::new(r);

    let mut si = stdin();
    let mut so = stdout();

    let mut bf = bf::interpreter::BFInterpreter::new(i, &mut si, &mut so);
    bf.run().unwrap();
}
