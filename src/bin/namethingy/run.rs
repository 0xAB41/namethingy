use std::fs::File;
use std::io::{self, BufRead};

use namethingy::NameGenerator;

use crate::args::*;

pub fn run() -> Result<(), ProgramArgError> {
    let args = ProgramArgs::parse()?;

    let mut name_generator = NameGenerator::with_order(args.order);

    let file = File::open(&args.corpus).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.flatten().filter(|s| !s.is_empty()).for_each(|w| {
        name_generator.train_word(&w);
    });
    for _i in 0..args.limit {
        println!("{}", name_generator.generate());
    }
    Ok(())
}