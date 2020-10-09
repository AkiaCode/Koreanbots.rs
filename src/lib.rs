use ureq::{self, json};
use std::time::Duration;

static BASE: &str = "https://api.koreanbots.dev";
static GET_BOTS: &str = "/bots/get";
static GET_VOTES_MYBOT: &str = "/bots/voted/";
static POST_BOT_SERVERS: &str = "/bots/servers";

///! It is a function that can get various information such as ranking of all bots list. (Page default is 1)
///!
///! # Examples
///!
///! ```
///! println!("{}", koreanbots_rs::get_bots("1"));
///! ```
pub fn get_bots(page: Option<&str>) -> ureq::SerdeValue {
    check();
    let resp = ureq::get(&(BASE.to_owned() + GET_BOTS + "?page=" + page.unwrap_or("1")))
        .set("content-type","application/json").call().into_json().unwrap();
    if resp["code"] == 200 {
        return resp["data"].to_owned(); 
    } else {
        return resp["message"].to_owned();
    }
}

///! This is a function that gets data from the bot you want to find
///!
///! # Examples
///!
///! ```
///! println!("{}", koreanbots_rs::get_seacrh_bots("387548561816027138"));
///! ```
pub fn get_seacrh_bots(id: &str) -> ureq::SerdeValue {
    check();
    let resp = ureq::get(&(BASE.to_owned() + GET_BOTS + "/" + id ))
        .set("content-type","application/json").call().into_json().unwrap();
    if resp["code"] == 200 {
        return resp["data"].to_owned(); 
    } else {
        return resp["message"].to_owned();
    }
}
///! This is a function that gets votes count from the bot.
///!
///! # Examples
///!
///! ```
///! println!("{}", koreanbots_rs::get_votes_mybot("MY_BOT_ID", "KOREANBOTS_TOKEN"));
///! ```
pub fn get_votes_mybot(id: &str, token: &str) -> ureq::SerdeValue {
    check();
    let resp = ureq::get(&(BASE.to_owned() + GET_VOTES_MYBOT + id ))
        .set("content-type","application/json")
        .set("token", token).call().into_json().unwrap();
    if resp["code"] == 200 {
        return resp["voted"].to_owned(); 
    } else {
        return resp["message"].to_owned();
    }
}

///! This is a function that sends the number of servers in the bot.
///!
///! # Examples
///!
///! ```
///! println!("{}", koreanbots_rs::post_bot_servers("KOREANBOTS_TOKEN", 123));
///! ```
pub fn post_bot_servers(token: &str, guilds : usize) -> ureq::SerdeValue {
    check();
    let resp = ureq::post(&(BASE.to_owned() + POST_BOT_SERVERS ))
        .set("token", token)
        .send_json(json!({ "servers":  guilds })).into_json().unwrap();
    if resp["code"] == 200 {
        return resp["servers"].to_owned(); 
    } else {
        return resp["message"].to_owned();
    }
}

fn check() {
    let resp = ureq::get(BASE)
    .set("content-type","application/json").call();

    if resp.header("x-ratelimit-remaining") == Some("0") || resp.header("x-ratelimit-remaining") >= resp.header("x-ratelimit-limit") {
        let secs_string = resp.header("x-ratelimit-reset").unwrap().to_string();
        let secs_number:u64 = secs_string.trim().parse().unwrap();
       return println!("you're now rate limited. retrying after {:?}ms", Duration::from_secs(secs_number));   
    }
}
