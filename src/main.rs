use koreanbots_rs::*;

fn main() {
    println!("{}", get_bots(Some("1"))[0]["name"]);
    println!("{}", get_seacrh_bots("387548561816027138")["web"]);
    println!("{}", get_votes_mybot("MY_BOT_ID", "KOREANBOTS_TOKEN"));
    println!("{}", post_bot_servers("KOREANBOTS_TOKEN", 1));
}
