extern crate strum;
#[macro_use]
extern crate strum_macros;

use forth::Forth;
use forth::builtins::Builtin;

fn main() {
    let mut f = Forth::new();
    println!("{}", f.eval("1 2 +").is_ok());
    println!("{:?}", f.stack());
}