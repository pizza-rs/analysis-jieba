//! Comprehensive tests for pizza-analysis-jieba (Jieba Chinese segmentation).

use pizza_analysis_jieba::JiebaTokenizer;
use pizza_engine::analysis::{AnalysisFactory, Token, Tokenizer};

// ═══════════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════════

fn terms(tokens: &[Token]) -> Vec<String> {
    tokens.iter().map(|t| t.term.to_string()).collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn tokenizer_construction() {
    let _t = JiebaTokenizer::new();
}

#[test]
fn tokenizer_clone() {
    let t1 = JiebaTokenizer::new();
    let _t2 = t1.clone();
}

// ═══════════════════════════════════════════════════════════════════════════════
// Basic segmentation
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn tokenize_simple_chinese() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("你今天很帅！");
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].term.as_ref(), "你");
    assert_eq!(tokens[1].term.as_ref(), "今天");
    assert_eq!(tokens[2].term.as_ref(), "很帅");
    assert_eq!(tokens[3].term.as_ref(), "！");
}

#[test]
fn tokenize_longer_sentence() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("我来到北京清华大学");
    let ts = terms(&tokens);
    assert!(!ts.is_empty());
    // Jieba in search mode should find sub-words like "清华", "大学", "清华大学"
    assert!(ts.iter().any(|s| s == "北京" || s == "清华" || s == "大学"));
}

#[test]
fn tokenize_mixed_chinese_english() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("我喜欢Python");
    let ts = terms(&tokens);
    assert!(!ts.is_empty());
    assert!(ts.iter().any(|s| s == "Python"));
}

#[test]
fn tokenize_chinese_with_numbers() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("今天是2024年");
    let ts = terms(&tokens);
    assert!(ts.iter().any(|s| s.contains("2024")));
}

#[test]
fn tokenize_pure_english() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("hello world");
    assert!(!tokens.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Edge cases
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn tokenize_empty_string() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("");
    assert!(tokens.is_empty());
}

#[test]
fn tokenize_single_chinese_char() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("我");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].term.as_ref(), "我");
}

#[test]
fn tokenize_single_ascii_char() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("a");
    assert_eq!(tokens.len(), 1);
}

#[test]
fn tokenize_whitespace() {
    let t = JiebaTokenizer::new();
    let _tokens = t.tokenize("   ");
    // Whitespace handling depends on Jieba
}

#[test]
fn tokenize_punctuation() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("，。！");
    // Chinese punctuation may be emitted as tokens
    assert!(!tokens.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Offsets and positions
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn offsets_are_byte_based() {
    let t = JiebaTokenizer::new();
    let text = "你今天很帅！";
    let tokens = t.tokenize(text);

    // First token "你" starts at byte 0, each Chinese char is 3 bytes
    assert_eq!(tokens[0].start_offset, 0);
    assert_eq!(tokens[0].end_offset, 3);

    // "今天" starts at byte 3
    assert_eq!(tokens[1].start_offset, 3);
    assert_eq!(tokens[1].end_offset, 9);
}

#[test]
fn positions_monotonic() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("自然语言处理是人工智能的分支");
    for i in 1..tokens.len() {
        assert!(
            tokens[i].position >= tokens[i - 1].position,
            "position must be monotonically increasing"
        );
    }
}

#[test]
fn positions_start_at_zero() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("中国");
    if !tokens.is_empty() {
        assert_eq!(tokens[0].position, 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Token content verification
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn tokens_cover_full_text() {
    let t = JiebaTokenizer::new();
    let text = "你好世界";
    let tokens = t.tokenize(text);
    // All terms joined should reconstruct the original text
    let joined: String = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert_eq!(joined, text);
}

#[test]
fn tokens_borrow_from_input() {
    let t = JiebaTokenizer::new();
    let text = "你好";
    let tokens = t.tokenize(text);
    for tok in &tokens {
        // Jieba tokenizer uses Cow::Borrowed
        assert!(matches!(tok.term, std::borrow::Cow::Borrowed(_)));
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_does_not_panic() {
    let mut factory = AnalysisFactory::new();
    pizza_analysis_jieba::register_all(&mut factory);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Long text / stress
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn tokenize_long_text() {
    let t = JiebaTokenizer::new();
    let text = "人工智能是计算机科学的一个分支，它企图了解智能的实质，并生产出一种新的能以人类智能相似的方式做出反应的智能机器，该领域的研究包括机器人、语言识别、图像识别、自然语言处理和专家系统等。";
    let tokens = t.tokenize(text);
    assert!(tokens.len() > 10);
}

#[test]
fn tokenize_repeated_chars() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("哈哈哈哈哈");
    assert!(!tokens.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unicode handling
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn tokenize_cjk_unified_ideographs() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("鬱鬱蔥蔥");
    assert!(!tokens.is_empty());
}

#[test]
fn tokenize_emoji_mixed() {
    let t = JiebaTokenizer::new();
    let _tokens = t.tokenize("你好😊世界🌍");
}

#[test]
fn tokenize_japanese_mixed() {
    let t = JiebaTokenizer::new();
    let tokens = t.tokenize("東京タワー");
    assert!(!tokens.is_empty());
}
