extern crate subprocess;

use std::io::{BufRead, BufReader};
use subprocess::Exec;

fn main() {
  let x = Exec::cmd("docker")
    .args(&[
      "run",
      "--rm",
      "-p",
      "80:80",
      "-P",
      "-it",
      "nginxdemos/hello",
    ])
    .stream_stdout()
    .unwrap();
  let br = BufReader::new(x);
  for (_, line) in br.lines().enumerate() {
    println!("{}\n\r", line.unwrap());
  }
}
