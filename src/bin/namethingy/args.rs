use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::path::Path;

use clap::{App, Arg, ArgMatches};

pub struct ProgramArgs {
    pub order: usize,
    pub limit: usize,
    pub corpus: String,
}

#[derive(Debug)]
pub struct ProgramArgError {
    pub kind: ProgramArgErrorKind
}

#[derive(Debug)]
pub enum ProgramArgErrorKind {
    NumberParseError,
    InvalidFilePath,
}

impl Display for ProgramArgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ProgramArgErrorKind::InvalidFilePath => write!(f, "Invalid file path"),
            ProgramArgErrorKind::NumberParseError => write!(f, "Invalid number")
        }
    }
}

impl From<std::num::ParseIntError> for ProgramArgError {
    fn from(_: ParseIntError) -> Self {
        ProgramArgError {
            kind: ProgramArgErrorKind::NumberParseError
        }
    }
}

impl ProgramArgs {
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

    pub fn parse() -> Result<ProgramArgs, ProgramArgError> {
        ProgramArgs::from_arg_matches(ProgramArgs::get_arg_matches())
    }

    fn from_arg_matches(arg_matches: ArgMatches) -> Result<ProgramArgs, ProgramArgError> {
        let order: usize = arg_matches.value_of("order").unwrap().parse()?;
        let limit: usize = arg_matches.value_of("limit").unwrap().parse()?;
        let corpus = arg_matches.value_of("corpus").unwrap().to_string();
        let corpus_path = Path::new(&corpus);
        if !(corpus_path.exists() && corpus_path.is_file()) {
            return Err(ProgramArgError {
                kind: ProgramArgErrorKind::InvalidFilePath
            });
        }
        Ok(ProgramArgs {
            order,
            limit,
            corpus,
        })
    }
}
