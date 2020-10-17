use core::time::Duration;


// REST API
macro_rules! api {
	($e:expr) => {
		concat!("https://api.beta.koreanbots.dev/v2", $e)
	};
}
macro_rules! bots {
	($e:expr) => {
		concat!("https://api.beta.koreanbots.dev/v2/bots", $e)
	};
}

fn _check(path: &str) {
    let resp = ureq::get(path)
    .set("content-type","application/json").call();

    if resp.header("x-ratelimit-remaining") == Some("0") || resp.header("x-ratelimit-remaining") >= resp.header("x-ratelimit-limit") {
        let secs_string = resp.header("x-ratelimit-reset").unwrap().to_string();
        let secs_number:u64 = secs_string.trim().parse().unwrap();
       return eprintln!("you're now rate limited. retrying after {:?}ms", Duration::from_secs(secs_number));
    }
}
pub mod rest;
