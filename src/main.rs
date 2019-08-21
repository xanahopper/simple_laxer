extern crate simple_laxer;
use self::simple_laxer::laxer::Laxer;

fn main() {
    let source = "int Xana = 10 + 5\n * 3 - main";
    let laxer = Laxer::new(source, false);
    for token in laxer {
        println!("{}", token)
    }
}
