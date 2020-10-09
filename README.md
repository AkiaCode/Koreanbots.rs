# koreanbots-rs


## Example

```rust
use koreanbots_rs::*;

fn main() {
    println!("{}", get_bots(Some("1"))[0]["name"]);
    // "원더봇"
    println!("{}", get_seacrh_bots("387548561816027138")["web"]);
    // "https://wonderbot.xyz"
    println!("{}", get_votes_mybot("MY_BOT_ID", "KOREANBOTS_TOKEN"));
    // {"message":"jwt malformed","name":"JsonWebTokenError"}
    println!("{}", post_bot_servers("KOREANBOTS_TOKEN", 1));
    // {"message":"jwt malformed","name":"JsonWebTokenError"}
    println!("{}", getbotsbycategory("관리", Some("1")));
    println!("{}", getserverwidget("387548561816027138"));
    println!("{}", getvotewidget("387548561816027138"));
    println!("{}", getsearchbots("원더봇", Some("1")));
}

```

License: Apache-2.0
