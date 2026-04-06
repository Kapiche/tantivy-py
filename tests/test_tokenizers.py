import tantivy
import pytest


class TestKapicheTokenizer:
    """Tests for kapiche_tokenizer()"""

    def test_kapiche_tokenizer_basic(self):
        analyzer = tantivy.kapiche_tokenizer()
        tokens = analyzer.analyze("Hello World")
        assert tokens == ["Hello", "World"]

    def test_kapiche_tokenizer_punctuation_removal(self):
        analyzer = tantivy.kapiche_tokenizer()
        tokens = analyzer.analyze("hello! world?")
        assert tokens == ["hello", "world"]

    def test_kapiche_tokenizer_preserves_hashtags(self):
        analyzer = tantivy.kapiche_tokenizer()
        tokens = analyzer.analyze("#hello @world")
        assert tokens == ["#hello", "@world"]

    def test_kapiche_tokenizer_possessive_removal(self):
        analyzer = tantivy.kapiche_tokenizer()
        tokens = analyzer.analyze("John's book")
        assert tokens == ["John", "book"]


class TestKapicheTokenizerLower:
    """Tests for kapiche_tokenizer_lower()"""

    def test_kapiche_tokenizer_lower_basic(self):
        analyzer = tantivy.kapiche_tokenizer_lower()
        tokens = analyzer.analyze("Hello World")
        assert tokens == ["hello", "world"]

    def test_kapiche_tokenizer_lower_punctuation(self):
        analyzer = tantivy.kapiche_tokenizer_lower()
        tokens = analyzer.analyze("Hello! World?")
        assert tokens == ["hello", "world"]

    def test_kapiche_tokenizer_lower_possessive(self):
        analyzer = tantivy.kapiche_tokenizer_lower()
        tokens = analyzer.analyze("John's book")
        assert tokens == ["john", "book"]


class TestKapicheTokenizerWithStopwords:
    """Tests for kapiche_tokenizer_lower_with_stopwords()"""

    def test_kapiche_tokenizer_lower_with_stopwords_english(self):
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        tokens = analyzer.analyze("The quick brown fox")
        assert tokens == ["quick", "brown", "fox"]

    def test_count_tokens_with_stopwords(self):
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        count = analyzer.count_tokens("the quick brown fox and a dog", unique=True)
        assert count == 4

    def test_stopwords_with_punctuation_and_possessives(self):
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        tokens = analyzer.analyze("John's the best!")
        assert tokens == ["john", "best"]

    def test_case_insensitive_stopword_removal(self):
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        tokens = analyzer.analyze("THE QUICK")
        assert tokens == ["quick"]

    def test_unicode_apostrophe_stopwords(self):
        """Test that stopwords with curly quotes and other unicode apostrophes are caught"""
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        # U+2019 RIGHT SINGLE QUOTATION MARK (curly quote)
        tokens = analyzer.analyze("I don\u2019t like purple elephants")
        assert tokens == ["like", "purple", "elephants"]
        # U+02BC MODIFIER LETTER APOSTROPHE
        tokens = analyzer.analyze("She can\u02BCt find green frogs")
        assert tokens == ["green", "frogs"]


class TestCountTokens:
    """Tests for TextAnalyzer.count_tokens()"""

    def test_count_tokens_basic(self):
        analyzer = tantivy.kapiche_tokenizer()
        count = analyzer.count_tokens("Hello World")
        assert count == 2

    def test_count_tokens_unique(self):
        analyzer = tantivy.kapiche_tokenizer_lower()
        total_count = analyzer.count_tokens("hello world hello")
        unique_count = analyzer.count_tokens("hello world hello", unique=True)
        assert total_count == 3
        assert unique_count == 2

    def test_count_tokens_empty(self):
        analyzer = tantivy.kapiche_tokenizer()
        count = analyzer.count_tokens("")
        assert count == 0


class TestUsagePattern:
    """Test the actual usage pattern mentioned in the requirements"""

    def test_token_counting_workflow(self):
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        text = "The quick brown fox jumps over the lazy dog"
        total = analyzer.count_tokens(text)
        unique = analyzer.count_tokens(text, unique=True)
        assert total > 0
        assert unique > 0
        assert unique <= total
