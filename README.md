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

License: Apache-2.0
