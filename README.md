# Practical use case for linked lists

This repository contains a toy validation library built in the `Practical use case for linked lists` article.

## Usage

```rust
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    jsonschema::validate(
      &json!({"another": 42}),
      &json!({
          "properties": {
              "another": {
                  "type": "string"
              }
          }
      }),
    ).expect_err("Should fail");
    Ok(())
}
// 42 is not of type 'string'
```

**NOTE**: To keep the article focused on path tracking, this library uses a single hardcoded schema.

Run benchmarks:

```sh
cargo bench
```

Build flamegraph for the "valid/10 levels" benchmark:

```sh
cargo flamegraph --bench jsonschema -o valid-10.svg -- --bench "valid/10 levels"
```

## Benchmarks

Here is the performance progression described in the article:

| Iteration       | Commit                                                                                                          | valid 0           | valid 5             | valid 10            | invalid 0          | invalid 5           | invalid 10          |
| --------------- | --------------------------------------------------------------------------------------------------------------- | ----------------- | ------------------- | ------------------- | ------------------ | ------------------- | ------------------- |
| No tracking     | [6173614](https://github.com/Stranger6667/article-linked-lists/commit/61736148cd2922722437805b547d2c865681d452) |            3.5 ns |             55.5 ns |            117.5 ns |            48.8 ns |               88 ns |              155 ns |
| Naive           | [485245b](https://github.com/Stranger6667/article-linked-lists/commit/485245bcba654cbaf08e5b82b53f2b8a8ba42540) | 4.1 ns (**x1.1**) |   279 ns (**x5.0**) | 677.6 ns (**x5.7**) | 95.4 ns (**x1.9**) | 387.2 ns (**x4.4**) | 812.3 ns (**x5.2**) |
| `imbl::Vector`  | [373fe66](https://github.com/Stranger6667/article-linked-lists/commit/373fe66fb33953ea3e4c244cb708cfddbe301104) | 5.2 ns (**x1.4**) | 224.6 ns (**x4.0**) | 663.8 ns (**x5.6**) | 88.8 ns (**x1.8**) | 399.0 ns (**x4.5**) | 966.2 ns (**x6.2**) |
| `&mut Vec`      | [2185462](https://github.com/Stranger6667/article-linked-lists/commit/218546263aa8c5692f024e225d87df211b159397) | 4.1 ns (**x1.1**) | 118.0 ns (**x2.1**) | 235.2 ns (**x2.0**) | 97.3 ns (**x1.9**) | 241.2 ns (**x2.7**) | 431.4 ns (**x2.7**) |
| Linked list     | [76df092](https://github.com/Stranger6667/article-linked-lists/commit/76df0929c2f40724daace9e5067cc786e43d39c2) | 3.6 ns (**x1.0**) |  68.1 ns (**x1.2**) | 132.2 ns (**x1.1**) | 95.9 ns (**x1.9**) | 254.1 ns (**x2.8**) | 455.7 ns (**x2.9**) |
| Tune capacity   | [06ae0c6](https://github.com/Stranger6667/article-linked-lists/commit/06ae0c61e5bd49e241f246fb14cbb519eb671c8e) | 3.7 ns (**x1.0**) |  64.7 ns (**x1.1**) | 122.4 ns (**x1.0**) | 90.4 ns (**x1.8**) | 200.1 ns (**x2.2**) | 352.2 ns (**x2.2**) |
| Single `Vec`    | [2ebd7a4](https://github.com/Stranger6667/article-linked-lists/commit/0f57d9348179c573feefcc47657f8cc625887135) | 3.7 ns (**x1.0**) |  67.3 ns (**x1.2**) | 129.7 ns (**x1.1**) | 77.5 ns (**x1.5**) | 184.2 ns (**x2.0**) | 340.1 ns (**x2.1**) |
