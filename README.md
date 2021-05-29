# koreanbots-rs

## Notice: Change koreanbots-rs to koreanbots

## Example

```rust
use koreanbots::blocking::Client;

fn main() {
    let client = Client::new("");
    println!("{:?}", client.get_bot("387548561816027138"));
}

```

``toml
koreanbot = { version = "2.0.1", features=["blocking"]}
```

License: Apache-2.0
