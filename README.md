<div align="center">

# 🇨🇳 pizza-analysis-jieba

**Jieba Chinese segmentation plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--jieba-blue)](https://github.com/pizza-rs/analysis-jieba)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

</div>

---

## Overview

`pizza-analysis-jieba` provides Chinese word segmentation using [jieba-rs](https://github.com/messense/jieba-rs) — the Rust port of the popular [Jieba](https://github.com/fxsjy/jieba) library.

### Key Features

- **Search Mode** — Fine-grained segmentation for search indexing
- **HMM New-Word Detection** — Identifies unknown words using Hidden Markov Model
- **User Dictionary** — Custom word lists with frequency and POS tagging
- **Fast & Memory-Efficient** — Rust-native implementation with prefix dictionary

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| Tokenizer | `jieba` | Jieba search-mode segmentation |
| Analyzer | `jieba` | jieba tokenizer → lowercase |

## Example

```text
Input:  "小明硕士毕业于中国科学院计算所"
Output: ["小明", "硕士", "毕业", "于", "中国", "科学", "学院", "科学院", "中国科学院", "计算", "计算所"]
```

## Installation

```toml
[dependencies]
pizza-analysis-jieba = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["jieba"] }
```

## Usage

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_jieba::register_all(&mut factory);
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
