use regex::Regex;
use std::io::{self, BufRead, BufWriter, Write};
use std::env;

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout);

    let mut expr = ".*";
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        expr = &args[1];
    }
    if let Ok(re) = Regex::new(expr) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(ln) = line {
                let strip_ln = strip_ansi_escapes::strip_str(ln.clone());
                if re.is_match(&strip_ln) {
                    writeln!(out, "{}", ln).unwrap();
                }
            }
        }
        out.flush().unwrap();
    } else {
        println!("Wrong regular expression : {}", expr);
    }
}
