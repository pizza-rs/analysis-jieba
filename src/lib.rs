#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::borrow::Cow;
use jieba_rs::Jieba;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::Tokenizer;

#[derive(Clone)]
pub struct JiebaTokenizer {
    jieba: Jieba,
}

impl JiebaTokenizer {
    pub fn new() -> Self {
        // External dictionary first (`<config>/analysis/jieba/dict.txt` — the
        // full default dictionary; deploy it with `make init-analysis-dicts`),
        // falling back to jieba's embedded default dictionary when built with
        // `default-dict`.
        #[cfg(feature = "std")]
        if let Some(path) = pizza_engine::analysis::dict::resolve("jieba", "dict.txt") {
            let f = std::fs::File::open(&path)
                .unwrap_or_else(|e| panic!("failed to open jieba dict {path:?}: {e}"));
            let mut reader = std::io::BufReader::new(f);
            let jieba = Jieba::with_dict(&mut reader)
                .unwrap_or_else(|e| panic!("failed to load jieba dict {path:?}: {e}"));
            return JiebaTokenizer { jieba };
        }
        #[cfg(feature = "default-dict")]
        {
            return JiebaTokenizer { jieba: Jieba::new() };
        }
        #[cfg(all(feature = "std", not(feature = "default-dict")))]
        panic!(
            "jieba dictionary not available: place the jieba dictionary at \
             <analysis dict dir>/jieba/dict.txt (make init-analysis-dicts), or \
             build pizza-analysis-jieba with the 'default-dict' feature"
        );
        // no_std without default-dict has no filesystem to read from; the
        // tokenizer degrades to whole-string tokens.
        #[cfg(all(not(feature = "std"), not(feature = "default-dict")))]
        JiebaTokenizer {
            jieba: Jieba::new(),
        }
    }
}

impl Tokenizer for JiebaTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        let mut indices = text.char_indices().collect::<Vec<_>>();
        indices.push((text.len(), '\0'));

        let orig_tokens = self
            .jieba
            .tokenize(text, jieba_rs::TokenizeMode::Search, true);
        let mut tokens = Vec::new();
        let mut position = 0;

        for token in orig_tokens {
            let start_offset = indices[token.start].0;
            let end_offset = indices[token.end].0;
            let term = Cow::Borrowed(&text[start_offset..end_offset]);

            tokens.push(Token {
                term,
                start_offset: start_offset as u32,
                end_offset: end_offset as u32,
                position,
            });
            position += 1;
        }

        tokens
    }
}

#[cfg(feature = "std")]
#[doc(hidden)]
/// Point the analysis dictionary directory at this crate's `data/` copy so
/// tests can construct [`JiebaTokenizer`] under any feature selection (the
/// external file must be laid out as `<dict_dir>/jieba/dict.txt`).
pub fn init_test_dict_dir() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!(
            "pizza-jieba-test-dict-{}",
            std::process::id()
        ));
        let ns = dir.join("jieba");
        if std::fs::create_dir_all(&ns).is_ok() {
            let _ = std::fs::copy(
                concat!(env!("CARGO_MANIFEST_DIR"), "/data/dict.txt"),
                ns.join("dict.txt"),
            );
        }
        pizza_engine::analysis::dict::set_dict_dir(&dir);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_jieba_tokenizing() {
        init_test_dict_dir();
        let tokenizer = JiebaTokenizer::new();

        let text = "你今天很帅！";
        let tokens = tokenizer.tokenize(text);

        println!("{:?}", tokens);

        assert_eq!(tokens.len(), 4);

        assert_eq!(tokens[0].term, Cow::Borrowed("你"));
        assert_eq!(tokens[0].start_offset, 0);
        assert_eq!(tokens[0].end_offset, 3);
        assert_eq!(tokens[0].position, 0);

        assert_eq!(tokens[1].term, Cow::Borrowed("今天"));
        assert_eq!(tokens[1].start_offset, 3);
        assert_eq!(tokens[1].end_offset, 9);
        assert_eq!(tokens[1].position, 1);

        assert_eq!(tokens[2].term, Cow::Borrowed("很帅"));
        assert_eq!(tokens[2].start_offset, 9);
        assert_eq!(tokens[2].end_offset, 15);
        assert_eq!(tokens[2].position, 2);

        assert_eq!(tokens[3].term, Cow::Borrowed("！"));
        assert_eq!(tokens[3].start_offset, 15);
        assert_eq!(tokens[3].end_offset, 18);
        assert_eq!(tokens[3].position, 3);
    }
}

pub mod register;
pub use register::register_all;
