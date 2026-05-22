# pizza-analysis-jieba

Chinese word segmentation for the [Pizza](https://github.com/infinilabs/pizza) search engine using [jieba-rs](https://github.com/messense/jieba-rs) — the Rust port of the popular [Jieba](https://github.com/fxsjy/jieba) Chinese text segmentation library.

## Components

| Name | Type | Description |
|------|------|-------------|
| `jieba` | Tokenizer | Chinese word segmentation (search mode) |
| `jieba` | Analyzer | Jieba tokenizer as a standalone analyzer |

## Usage

### Full Analyzer

```json
{
  "analyzer": {
    "type": "jieba"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "jieba",
    "filter": ["lowercase", "cjk_width"]
  }
}
```

### Example

**Input:** `小明硕士毕业于中国科学院计算所`

**Output tokens:** `小明`, `硕士`, `毕业`, `于`, `中国`, `科学`, `学院`, `科学院`, `中国科学院`, `计算`, `计算所`

## Algorithm

Jieba uses a combination of:

1. **Dictionary-based segmentation** — Uses a prefix dictionary (~350K entries) to build a directed acyclic graph (DAG) of possible word boundaries
2. **Dynamic programming** — Finds the path through the DAG that maximizes total word frequency
3. **HMM for unknown words** — Uses a Hidden Markov Model (Viterbi algorithm) to segment words not in the dictionary

### Search Mode

In search mode (the default for this crate), long words are further segmented into shorter sub-words for better search recall. For example, `中国科学院` produces both the full word and its constituents `中国`, `科学`, `学院`, `科学院`.

## Comparison with Other Chinese Tokenizers

| Feature | Jieba | SmartCN | IK |
|---------|-------|---------|-----|
| Dictionary size | ~350K | 85K | 275K |
| Unknown words | HMM-based | None | Limited |
| Search mode | Yes (sub-word expansion) | No | Max-word mode |
| TF-IDF / TextRank | Built-in | No | No |
| Algorithm | DP + HMM | Viterbi DP | Dictionary + heuristics |

## Data Sources

- **Dictionary**: jieba's default dictionary (~350K Chinese words with frequencies), embedded via the `default-dict` feature of `jieba-rs`
- **HMM model**: Pre-trained character transition probabilities for unknown word detection
- **Source**: [fxsjy/jieba](https://github.com/fxsjy/jieba) (MIT license)

## Features

Built-in support for:
- TF-IDF keyword extraction
- TextRank keyword extraction
- Custom dictionary loading at runtime

## License

MIT
