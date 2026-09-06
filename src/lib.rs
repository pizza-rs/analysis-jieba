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
        // External user dictionary first (`<config>/analysis/jieba/dict.txt`),
        // falling back to jieba's embedded default dictionary.
        #[cfg(feature = "std")]
        if let Some(path) = pizza_engine::analysis::dict::resolve("jieba", "dict.txt") {
            let f = std::fs::File::open(&path)
                .unwrap_or_else(|e| panic!("failed to open jieba dict {path:?}: {e}"));
            let mut reader = std::io::BufReader::new(f);
            let jieba = Jieba::with_dict(&mut reader)
                .unwrap_or_else(|e| panic!("failed to load jieba dict {path:?}: {e}"));
            return JiebaTokenizer { jieba };
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_jieba_tokenizing() {
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
