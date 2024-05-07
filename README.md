# Practical use case for linked lists

This repository contains a toy validation library built in the `Practical use case for linked lists` article.

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
```
