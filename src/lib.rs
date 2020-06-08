use std::collections::HashMap;
use std::hash::Hash;

use rand::{Rng, thread_rng};

type Token = Option<char>;
type Ngram = Vec<Token>;
type FrequencyMap<T> = HashMap<T, usize>;

trait FrequencyMapOps<T> {
    fn incr(&mut self, key: T);
    fn count(&self) -> usize;
}

impl<T: Eq + Hash> FrequencyMapOps<T> for FrequencyMap<T> {
    fn incr(&mut self, key: T) {
        let count = self.entry(key).or_insert(0);
        *count += 1;
    }

    fn count(&self) -> usize {
        self.values().sum()
    }
}

trait MapRandom<T> {
    fn choose_weighted(&self) -> &T;
}

impl<T> MapRandom<T> for FrequencyMap<T> {
    fn choose_weighted(&self) -> &T {
        let mut rng = thread_rng();
        let total_tokens: usize = self.values().sum();
        let chosen_token = rng.gen_range(0, total_tokens + 1);
        let mut sum = 0;
        for (key, value) in self.iter() {
            sum += *value;
            if sum >= chosen_token {
                return key;
            }
        }
        //FIXME: this is recheable if order is say 100
        unreachable!()
    }
}

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

    pub fn generate(&self) -> String {
        let mut ngram = self.prefix_ngrams.choose_weighted().clone();
        let mut chars: Vec<char> = ngram.iter().flatten().copied().collect();
        while let Some(next_states) = self.state_map.get(&ngram) {
            if let Some(c) = next_states.choose_weighted() {
                ngram = ngram[1..].to_vec();
                ngram.push(Some(*c));
                chars.push(*c);
            } else {
                break;
            }
        }
        chars.iter().collect::<String>()
    }
}
