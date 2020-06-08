use std::process;

mod run;
mod args;

fn main() {
    match run::run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
