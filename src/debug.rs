pub mod unreleased;


/*
  * When using custom flag:
    RUSTFLAGS="--cfg imm" cargo run --example macro_with_cfg 
*/
#[macro_export]
macro_rules! dp {
  ($buf:expr, $format:expr) => {
    if cfg!(imm) {
      writeln!($buf,$format).unwrap();
    } else {
      println!($format);
    }
  };
  ($buf:expr, $format:expr, $($var:expr),*) => {
    if cfg!(imm) {
      writeln!($buf,$format,$($var),*).unwrap();
    } else {
      println!($format,$($var),*);
    }
  }
}

#[macro_export]
macro_rules! fastp {
  ($buf:expr, $format:expr) => {
    if cfg!(debug_assertions) {
      println!($format);
    } else {
      writeln!($buf,$format).unwrap();
    }
  };
  ($buf:expr, $format:expr, $($var:expr),*) => {
    if cfg!(debug_assertions) {
      println!($format,$($var),*);
    } else {
      writeln!($buf,$format,$($var),*).unwrap();
    }
  }
}
