use std::io::*;
use StudyRustMacro::*;

fn main() {
    // NOTE: because stdout().lock() is not allowed
    let stdout = stdout();
    let lock = stdout.lock();
    let mut buf = BufWriter::new(lock);

  fastp!(buf,"A");
  fastp!(buf,"A {} {}",1,2);
  buf.flush().unwrap();
  dp!(buf,"A");
  dp!(buf,"A {} {}",1,"X");
  buf.flush().unwrap();
}
