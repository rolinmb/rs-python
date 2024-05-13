use std::fs::File;
use std::io::prelude::*;
//use std::io::{BufRead, BufReader};
use std::process::{Command/*, Stdio*/};

fn main() {
  let floats: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
  println!("(main.rs) floats: Vec<f64> = {:?}\n", floats);
  let python_str = format!("import matplotlib.pyplot as plt\nif __name__ == \"__main__\":\n\tfloats = {:?}\n\tprint(\"\\n(main.py) floats: Vec<f64> from src/main.rs =\")\n\tprint(floats)\n\tfig = plt.plot(floats)\n\tplt.xlabel(\"Index\")\n\tplt.ylabel(\"Value\")\n\tplt.ylabel(\"Simple Plot\")\n\tplt.show()", floats);
  let mut file = File::create("src/main.py").expect("Unable to create src/main.py");
  file.write_all(python_str.as_bytes()).expect("(main.rs) Unable to write python_str to src/main.py");
  let output = Command::new("python").arg("src/main.py").output().expect("(main.rs) Failed to execute src/main.py");
  println!("(main.rs) Output from src/main.py ~~~\n{}\n~~~", String::from_utf8_lossy(&output.stdout));
  /*let process = Command::new("python").arg("-v").stdout(Stdio::piped()).spawn().expect("(main.rs) Failed to start python -v verbose mode");
  if let Some(ref stdout) = process.stdout {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
      println!("{}", line.expect("(main.rs) failed to read line from python -v verbose mode"));
    }
  }
  let _ = process.wait().expect("(main.rs) Failed to wait for child process python -v verbose mode");*/
}
