use std::collections::HashSet;

use super::Analyzer;

#[derive(Debug)]
pub struct WordSearch {
    word: String,
    paths: HashSet<String>,
}

impl WordSearch {
    pub fn new(word: String) -> Self {
        Self {
            word,
            paths: HashSet::default(),
        }
    }

    pub fn paths(&self) -> HashSet<String> {
        self.paths.clone()
    }

    pub fn num_paths(&self) -> usize {
        self.paths.len()
    }
}

impl Analyzer for WordSearch {
    fn process_chunk(&mut self, path: &str, chunk: &str) {
        if chunk.contains(&self.word) {
            self.paths.insert(path.to_owned());
        }
    }
}
