use pyo3::prelude::*;
use tantivy::tokenizer as tvt;

/// Tantivy's TextAnalyzer
///
/// Do not instantiate this class directly.
/// Use the factory functions like kapiche_tokenizer() instead.
#[derive(Clone)]
#[pyclass(module = "tantivy.tantivy")]
pub(crate) struct TextAnalyzer {
    pub(crate) analyzer: tvt::TextAnalyzer,
}

#[pymethods]
impl TextAnalyzer {
    /// Tokenize a string
    /// Args:
    /// - text (string): text to tokenize.
    /// Returns:
    /// - list(string): a list of tokens/words.
    fn analyze(&mut self, text: &str) -> Vec<String> {
        let mut token_stream = self.analyzer.token_stream(text);
        let mut tokens = Vec::new();

        while token_stream.advance() {
            tokens.push(token_stream.token().text.clone());
        }
        tokens
    }

    /// Count tokens without materializing them into a collection.
    /// Much faster than len(analyze(text)) for large texts.
    ///
    /// Args:
    /// - text (string): text to analyze
    /// - unique (bool, optional): if True, count only unique tokens. Defaults to False.
    /// Returns:
    /// - int: count of tokens (unique or total based on parameter)
    #[pyo3(signature = (text, unique=false))]
    fn count_tokens(&mut self, text: &str, unique: bool) -> usize {
        let mut token_stream = self.analyzer.token_stream(text);

        if unique {
            let mut seen = std::collections::HashSet::new();
            while token_stream.advance() {
                seen.insert(token_stream.token().text.clone());
            }
            seen.len()
        } else {
            let mut count = 0;
            while token_stream.advance() {
                count += 1;
            }
            count
        }
    }
}
