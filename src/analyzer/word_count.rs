use std::collections::HashMap;

use super::Analyzer;

#[derive(Debug)]
pub struct WordCount {
    words: HashMap<String, usize>,
}

impl WordCount {
    pub fn new() -> Self {
        Self {
            words: HashMap::default(),
        }
    }

    pub fn n_most_frequent(&self, num: usize) -> Vec<(String, usize)> {
        let mut x: Vec<_> = self.words.clone().into_iter().collect();
        x.sort_by(|a, b| b.1.cmp(&a.1));
        x.into_iter().take(num).collect()
    }

    pub fn n_unique_words(&self) -> usize {
        self.words.keys().map(|k| k.to_owned()).count()
    }
}

impl Analyzer for WordCount {
    fn process_chunk(&mut self, path: &str, chunk: &str) {
        chunk.split(" ").for_each(|x| {
            self.words
                .entry(x.to_owned())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
    }
}
