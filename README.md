# Blazingly Fast Linked Lists

This repository contains a toy validation library built in the [Blazingly Fast Linked Lists](https://dygalo.dev/blog/blazingly-fast-linked-lists/) article.

## Usage

```rust
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    jsonschema::validate(
      // JSON instance to validate
      &json!({
          "name": "John",
          "location": {
               "country": 404
          }
      }),
      // JSON schema
      &json!({
          "properties": {
              "name": {
                  "type": "string"
              },
              "location": {
                  "properties": {
                      "country": {
                          "type": "string"
                      }
                  }
              }
          }
      }),
    ).expect_err("Should fail");
    Ok(())
}
// 404 is not of type ‘string’ at /location/country
```

**NOTE**: To keep the article focused on path tracking, this library uses a single hardcoded schema.

Run benchmarks:

```sh
cargo bench
```

Build flame graph for the "valid/10 levels" benchmark:

```sh
cargo flamegraph --bench jsonschema -o valid-10.svg -- --bench "valid/10 levels"
```

## Benchmarks

![performance-comparison](https://github.com/Stranger6667/article-linked-lists/assets/1236561/b714d431-cb4b-4c3b-9448-d0acac1c29d9)

Here is the performance progression described in the article:

| Iteration       | Commit                                                                                                          | valid 0           | valid 5             | valid 10            | invalid 0          | invalid 5           | invalid 10          |
| --------------- | --------------------------------------------------------------------------------------------------------------- | ----------------- | ------------------- | ------------------- | ------------------ | ------------------- | ------------------- |
| No tracking     | [a030dcb](https://github.com/Stranger6667/article-linked-lists/commit/a030dcb18448555efa1a8f63f8b5ccebef7d2f59) |            36.3 µs |             553.8 µs |            1.11 ms |            475.2 µs |               914.8 µs |              1.48 ms |
| Naive           | [9ef7b4c](https://github.com/Stranger6667/article-linked-lists/commit/9ef7b4c56c8ca2ba3dcd15681daff6951aa64c2c) | 40.9 µs (**+12.0%**) |   2.61 ms (**+369.4%**) | 6.69 ms (**+499.6%**) | 961.2 µs (**+100.8%**) | 4.11 ms (**+346.8%**) | 9.07 ms (**+502.7%**) |
| `imbl::Vector`  | [77adb2c](https://github.com/Stranger6667/article-linked-lists/commit/77adb2c34ef95b978e90429c80bad59a422caa39) | 47.1 µs (**+13.6%**) | 2.25 ms (**-15.6%**) | 6.49 ms (**-3.7%**) | 904.3 µs (**-6.6%**) | 4.09 ms (**-1.2%**) | 9.77 ms (**+6.7%**) |
| `&mut Vec`      | [7c94736](https://github.com/Stranger6667/article-linked-lists/commit/7c9473689bf24b90c8e0c45f700ad985b536a73e) | 40.2 µs (**-13.1%**) | 1.24 ms (**-46.0%**) | 2.46 ms (**-62.3%**) | 951.7 µs (**+3.0%**) | 2.39 ms (**-42.2%**) | 4.16 ms (**-57.8%**) |
| Linked list     | [91ec92c](https://github.com/Stranger6667/article-linked-lists/commit/91ec92c757d3948a2a032a55d035e6fadc63fdcf) | 35.0 µs (**-14.8%**) |  663.5 µs (**-46.4%**) | 1.32 ms (**-46.6%**) | 958.9 µs (**+1.8%**) | 2.54 ms (**+5.1%**) | 4.58 ms (**+9.9%**) |
| Tune capacity   | [10ae4f1](https://github.com/Stranger6667/article-linked-lists/commit/10ae4f100935c757bb7707defcf122c179aee2dc) | 39.1 µs (**+11.2%**) |  667.9 µs (**+0.5%**) | 1.30 ms (**-1.7%**) | 899.7 µs (**-7.5%**) | 1.96 ms (**-23.3%**) | 3.49 ms (**-24.3%**) |
| Single `Vec`    | [d3d2182](https://github.com/Stranger6667/article-linked-lists/commit/d3d2182e00aba996134475b90e87d565dfe47ac3) | 39.3 µs (**-0.2%**) |  652.3 µs (**-2.7%**) | 1.35 ms (**+2.2%**) | 765.1 µs (**-14.2%**) | 1.83 ms (**-6.9%**) | 3.33 ms (**-5.9%**) |
