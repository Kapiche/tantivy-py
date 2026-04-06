mod filter_constants;
pub mod outer_punctuation_filter;
pub mod possessive_contraction_filter;

use filter_constants::STOPWORDS_EN;

/// Unicode apostrophe characters to expand stopwords with.
const APOSTROPHES: [char; 8] = [
    '\u{0027}', // ' - Apostrophe
    '\u{2019}', // ' - Right single quotation mark
    '\u{02BC}', // ʼ - Modifier letter apostrophe
    '\u{02BB}', // ʻ - Modifier letter turned comma
    '\u{055A}', // ՚ - Armenian apostrophe
    '\u{A78B}', // Ꞌ - Latin capital letter saltillo
    '\u{A78C}', // ꞌ - Latin small letter saltillo
    '\u{FF07}', // ＇ - Fullwidth apostrophe
];

/// Check if a string contains any apostrophe character.
fn contains_apostrophe(s: &str) -> bool {
    s.chars().any(|c| APOSTROPHES.contains(&c))
}

/// Replace all apostrophe variants with a specific apostrophe character.
fn replace_apostrophes(s: &str, replacement: char) -> String {
    s.chars()
        .map(|c| {
            if APOSTROPHES.contains(&c) {
                replacement
            } else {
                c
            }
        })
        .collect()
}

/// Expand a stopword list to include all apostrophe variants.
///
/// For each stopword containing an apostrophe, generates versions
/// with every unicode apostrophe variant.
///
/// Example: "don't" becomes ["don't", "don\u{2019}t", "don\u{02BC}t", ...]
fn expand_stopwords_with_apostrophe_variants(
    base_stopwords: &[&str],
) -> Vec<String> {
    let mut expanded = Vec::with_capacity(base_stopwords.len());

    for word in base_stopwords {
        if contains_apostrophe(word) {
            for &apos in &APOSTROPHES {
                expanded.push(replace_apostrophes(word, apos));
            }
        } else {
            expanded.push(word.to_string());
        }
    }

    expanded
}

/// Get the Kapiche custom English stopwords list with apostrophe variants expanded.
pub fn get_stopwords_filter_en() -> Vec<String> {
    expand_stopwords_with_apostrophe_variants(&STOPWORDS_EN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_apostrophe() {
        assert!(contains_apostrophe("don't"));
        assert!(contains_apostrophe("don\u{2019}t"));
        assert!(contains_apostrophe("don\u{02BC}t"));
        assert!(!contains_apostrophe("hello"));
    }

    #[test]
    fn test_replace_apostrophes() {
        assert_eq!(replace_apostrophes("don\u{2019}t", '\''), "don't");
        assert_eq!(replace_apostrophes("don\u{02BC}t", '\''), "don't");
        assert_eq!(replace_apostrophes("hello", '\''), "hello");
    }

    #[test]
    fn test_expand_stopwords_no_apostrophes() {
        let base = vec!["hello", "world"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        assert_eq!(expanded, vec!["hello", "world"]);
    }

    #[test]
    fn test_expand_stopwords_with_apostrophe_word() {
        let base = vec!["don't"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        assert_eq!(expanded.len(), 8);
        assert!(expanded.contains(&"don't".to_string()));
        assert!(expanded.contains(&"don\u{2019}t".to_string()));
        assert!(expanded.contains(&"don\u{02BC}t".to_string()));
    }

    #[test]
    fn test_expand_stopwords_mixed() {
        let base = vec!["hello", "don't", "world"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        // hello + 8 variants of don't + world = 10
        assert_eq!(expanded.len(), 10);
        assert!(expanded.contains(&"hello".to_string()));
        assert!(expanded.contains(&"world".to_string()));
    }

    #[test]
    fn test_get_stopwords_filter_en_has_apostrophe_variants() {
        let stopwords = get_stopwords_filter_en();
        assert!(stopwords.contains(&"don't".to_string()));
        assert!(stopwords.contains(&"don\u{2019}t".to_string()));
        assert!(stopwords.contains(&"can't".to_string()));
        assert!(stopwords.contains(&"can\u{2019}t".to_string()));
    }
}
