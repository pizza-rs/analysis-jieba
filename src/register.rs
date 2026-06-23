//! Register Jieba analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;

use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;

use crate::JiebaTokenizer;

/// Register Jieba tokenizer and analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_tokenizer_with("jieba", || Box::new(JiebaTokenizer::new()));

    factory.register_analyzer_with(
        "jieba",
        || Analyzer::new(vec![], Box::new(JiebaTokenizer::new()), vec![]),
    );
}
