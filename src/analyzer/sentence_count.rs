use super::Analyzer;

#[derive(Debug)]
pub struct SentenceCount {
    min_words: usize,
    max_words: usize,
    n_sentence: u64,
}

impl SentenceCount {
    pub fn new(min_words: usize, max_words: usize) -> Self {
        Self {
            n_sentence: 0,
            min_words,
            max_words,
        }
    }

    pub fn n_sentence(&self) -> u64 {
        self.n_sentence
    }
}

impl Analyzer for SentenceCount {
    fn process_chunk(&mut self, _path: &str, chunk: &str) {
        let words = chunk.split(' ').count();
        if words > self.min_words && words < self.max_words {
            self.n_sentence += 1
        }
    }
}
