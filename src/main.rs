extern crate nom;

pub mod vm;
pub mod instructions;
pub mod repl;
pub mod assembler;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
