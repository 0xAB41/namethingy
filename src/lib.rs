use std::collections::HashMap;
mod map;
use map::*;

type Token = Option<char>;
type Ngram = Vec<Token>;


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

    pub fn generate(&self) -> Option<String> {
        if let Some(prefix) = self.prefix_ngrams.choose_weighted().ok() {
            let mut ngram = prefix.clone();
            let mut chars: Vec<char> = ngram.iter().flatten().copied().collect();
            while let Some(next_states) = self.state_map.get(&ngram) {
                if let Some(Some(c)) = next_states.choose_weighted().ok() {
                    ngram = ngram[1..].to_vec();
                    ngram.push(Some(*c));
                    chars.push(*c);
                } else {
                    break;
                }
            }

            Some(chars.iter().collect::<String>())
        } else {
            None
        }
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.name_generator.generate()
    }
}

#[cfg(test)]
mod tests {
   use super::*;
}
