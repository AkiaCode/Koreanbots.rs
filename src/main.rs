use koreanbots_rs::*;

fn main() {
    println!("{}", get_bots(Some("1"))[0]["name"]);
    println!("{}", get_bot("387548561816027138")["web"]);
    println!("{}", get_votes_mybot("MY_BOT_ID", "KOREANBOTS_TOKEN"));
    println!("{}", post_bot_servers("KOREANBOTS_TOKEN", 1));
    println!("{}", get_bots_by_category("관리", Some("1")));
    println!("{}", get_server_widget("387548561816027138"));
    println!("{}", get_vote_widget("387548561816027138"));
    println!("{}", get_search_bots("원더봇", Some("1")));
}
