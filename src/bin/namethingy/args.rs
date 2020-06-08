use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::path::Path;

use clap::{App, Arg, ArgMatches};

pub struct Args {
    pub order: usize,
    pub limit: usize,
    pub corpus: String,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind
}

#[derive(Debug)]
pub enum ErrorKind {
    NumberParseError,
    InvalidFilePath,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::InvalidFilePath => write!(f, "Invalid file path"),
            ErrorKind::NumberParseError => write!(f, "Invalid number")
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error {
            kind: ErrorKind::NumberParseError
        }
    }
}

impl Args {
    fn get_arg_matches<'a>() -> ArgMatches<'a> {
        App::new("NameThingy")
            .version("0.2")
            .arg(Arg::with_name("order")
                .short("o")
                .long("order")
                .help("Set order")
                .takes_value(true)
                .default_value("2"))
            .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .help("Number of names to generate")
                .takes_value(true)
                .default_value("10"))
            .arg(Arg::with_name("corpus")
                .short("c")
                .long("corpus")
                .value_name("FILE")
                .help("Corpus")
                .takes_value(true)
                .required(true))
            .get_matches()
    }

    pub fn parse() -> Result<Args, Error> {
        Args::from_arg_matches(Args::get_arg_matches())
    }

    fn from_arg_matches(arg_matches: ArgMatches) -> Result<Args, Error> {
        let order: usize = arg_matches.value_of("order").unwrap().parse()?;
        let limit: usize = arg_matches.value_of("limit").unwrap().parse()?;
        let corpus = arg_matches.value_of("corpus").unwrap().to_string();
        let corpus_path = Path::new(&corpus);
        if !(corpus_path.exists() && corpus_path.is_file()) {
            return Err(Error {
                kind: ErrorKind::InvalidFilePath
            });
        }
        Ok(Args {
            order,
            limit,
            corpus,
        })
    }
}
