pub mod sentence_count;
pub mod word_count;
pub mod word_search;

pub trait Analyzer {
    fn process_chunk(&mut self, path: &str, chunk: &str);
}
