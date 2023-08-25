mod tokenize;
mod parse;
mod execute;

use std::io::{self, BufRead};

use tokenize::tokenize;
use parse::parse;
use execute::{execute, Res};

pub fn main() {
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    match process(line.unwrap()) {
      Ok(Res::Exit) => handle_exit(),
      Ok(Res::Success(msg)) => handle_success(msg),
      Err(what) => handle_failure(what),
    }
  }
}

fn process(line: String) -> Result<execute::Res, String> {
  tokenize(line)
    .and_then(parse)
    .and_then(execute)
}

fn handle_exit() {
  println!("bye :)");
  std::process::exit(0)
}

fn handle_success(msg: String) {
  println!("success: {}", msg)
}

fn handle_failure(msg: String) {
  println!("failure: {}", msg)
}
