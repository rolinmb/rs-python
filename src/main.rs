//use std::fs::File;
//use std::io::prelude::*;
//use std::io::{BufRead, BufReader};
use std::process::{Command/*, Stdio*/};

fn call_matplotlib_simple(data: Vec<f64>, title: String, xlabel: String, ylabel: String) {
  let python_str = format!(
    "import matplotlib.pyplot as plt\nif __name__ == \"__main__\":\n\tdata = {:?}\n\tprint(\"\\n(main.py) data: Vec<f64> from src/main.rs =\")\n\tprint(data)\n\tplt.plot(data)\n\tplt.title(\"{}\")\n\tplt.xlabel(\"{}\")\n\tplt.ylabel(\"{}\")\n\tplt.show()",
    data, title, xlabel, ylabel,
  );
  //let mut file = File::create("src/main.py").expect("(main.rs) :: call_matplotlib() Unable to write python_str to src/main.py");
  //file.write_all(python_str.as_bytes()).expect("(main.rs) :: call_matplotlib() Unable to write python_str to src/main.py");
  let pycmd = if std::env::consts::OS == "windows" {
    Command::new("python")
  } else {
    Command::new("python3")
  }
  .arg("-c")
  .arg(&python_str)
  .output()
  .expect("(main.rs) :: call_matplotlib() Failed to execute src/main.py");
  println!("(main.rs) :: call_matplotlib() Output from src/main.py ~~~\n{}\n~~~", String::from_utf8_lossy(&pycmd.stdout));
}

/*fn open_python_cli() {
  let process = Command::new("python").arg("-v").stdout(Stdio::piped()).spawn().expect("(main.rs) Failed to start python -v verbose mode");
  if let Some(ref stdout) = process.stdout {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
      println!("{}", line.expect("(main.rs) failed to read line from python -v verbose mode"));
    }
  }
  let _ = process.wait().expect("(main.rs) Failed to wait for child process python -v verbose mode");
}*/

fn main() {
  let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
  println!("(main.rs) :: main() data: Vec<f64> = {:?}\n", data);
  call_matplotlib_simple(data, "Matplotlib rs-python Test".to_string(), "Index".to_string(), "Value".to_string());
  //open_python_cli();
}
