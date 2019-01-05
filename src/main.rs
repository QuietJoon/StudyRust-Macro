use std::io::*;
use StudyRustMacro::*;

fn main() {
    // NOTE: because stdout().lock() is not allowed
    let stdout = stdout();
    let lock = stdout.lock();
    let mut buf = BufWriter::new(lock);

    fastp!(buf,"A");
    fastp!(buf,"B {}",1);
    fastp!(buf,"C {} {}",1,2);

    dop!(buf,"D {} {}",1,2);
    dopl!(buf,"E {} {}",1,2);
}
