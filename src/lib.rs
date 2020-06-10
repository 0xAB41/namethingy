use std::collections::HashMap;
mod map;
use map::*;
use std::fmt::{Display, Formatter};

type Token = Option<char>;
type Ngram = Vec<Token>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
    Unknown
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Empty => write!(f, "Generator is not trained/empty"),
            Error::Unknown => write!(f, "Unknown error")
        }
    }
}

impl From<MapRandomError> for Error {
    fn from(m: MapRandomError) -> Self {
        match m {
            MapRandomError::EmptyMap => Error::Empty,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct NameGenerator {
    prefix_ngrams: FrequencyMap<Ngram>,
    state_map: HashMap<Ngram, FrequencyMap<Token>>,
    order: usize,
}

impl Default for NameGenerator {
    fn default() -> Self {
        NameGenerator::new()
    }
}

impl NameGenerator {
    pub fn with_order(order: usize) -> Self {
        NameGenerator {
            prefix_ngrams: FrequencyMap::new(),
            state_map: HashMap::new(),
            order,
        }
    }

    pub fn new() -> Self {
        NameGenerator::with_order(2)
    }

    pub fn train(&mut self, words: &[&str]) -> &mut Self {
        for word in words {
            self.train_word(&word);
        }
        self
    }

    pub fn train_word(&mut self, word: &str) -> &mut Self {
        let unigrams: Ngram = word.chars().map(Some).collect();
        let window = self.order;
        for (i, ngram) in (0..).zip(unigrams.windows(window)) {
            let ngram = ngram.to_vec();
            if i == 0 {
                self.prefix_ngrams.incr(ngram.clone());
            }

            let next_pos = i + window;
            let next = unigrams.get(next_pos).copied().flatten();

            let ngram_next_states = self.state_map.entry(ngram).or_insert_with(FrequencyMap::<Token>::new);
            ngram_next_states.incr(next);
        }
        self
    }

    pub fn generate(&self) -> Result<String> {
        let mut ngram = self.prefix_ngrams.choose_weighted()?.clone();
        let mut chars: Vec<char> = ngram.iter().flatten().copied().collect();
        while let Some(next_states) = self.state_map.get(&ngram) {
            if let Some(c) = next_states.choose_weighted()? {
                ngram = ngram[1..].to_vec();
                ngram.push(Some(*c));
                chars.push(*c);
            } else {
                break;
            }
        }

        Ok(chars.iter().collect::<String>())
    }

    pub fn iter(&self) -> Iter {
        Iter {
            name_generator: self
        }
    }
}

pub struct Iter<'a> {
    name_generator: &'a NameGenerator
}

impl Iterator for Iter<'_> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.name_generator.generate())
    }
}

#[cfg(test)]
mod tests {
   use super::*;
}
