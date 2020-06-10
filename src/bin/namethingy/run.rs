use std::fs::File;
use std::io::{self, BufRead};

use namethingy::NameGenerator;

use crate::args::*;

pub fn run() -> Result<(), Error> {
    let args = Args::parse()?;

    let mut name_generator = NameGenerator::with_order(args.order);

    let file = File::open(&args.corpus).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.flatten().filter(|s| !s.is_empty()).for_each(|w| {
        name_generator.train_word(&w);
    });

    name_generator
        .iter()
        .take(args.limit)
        .for_each(|x| println!("{}", x));
    Ok(())
}