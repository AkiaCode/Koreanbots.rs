pub mod blocking;
pub mod model;

const BASE_URL: &str = "https://koreanbots.dev/api/v2";

use std::collections::HashMap;

use model::UserInfo;
use reqwest::{
    header::HeaderMap,
    header::{AUTHORIZATION, CONTENT_TYPE},
    Url,
};

use crate::model::{
    response::{Data, Response, ResponseUpdate},
    Bot, VoteCheck, WidgetQuery, WidgetType,
};

pub struct Client {
    authorization: &'static str,
}

impl Client {
    pub fn new(authorization: &'static str) -> Self {
        Self { authorization }
    }

    pub async fn get_bot(&self, bot_id: &str) -> Response<Bot> {
        let mut url = String::from("bots/");
        url.push_str(bot_id);
        let fetch = Client::_fetch("GET", &url, None, None, None);
        let json: Response<Bot> = serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn search_bot(&self, search: &str, page: Option<usize>) -> Response<Data<Bot>> {
        let mut query = HashMap::new();
        query.insert("query", search.to_string());
        if let Some(page) = page {
            query.insert("page", page.to_string());
        } else {
            query.insert("page", "1".to_string());
        }
        let fetch = Client::_fetch("GET", "search/bots", None, Some(query), None);

        let json: Response<Data<Bot>> =
            serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn get_votes_list(&self, page: Option<usize>) -> Response<Data<Bot>> {
        let mut query = HashMap::new();
        if let Some(page) = page {
            query.insert("page", page.to_string());
        } else {
            query.insert("page", "1".to_string());
        }
        let fetch = Client::_fetch("GET", "list/bots/votes", None, Some(query), None);

        let json: Response<Data<Bot>> =
            serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn get_new_bot_list(&self) -> Response<Data<Bot>> {
        let fetch = Client::_fetch("GET", "list/bots/new", None, None, None);

        let json: Response<Data<Bot>> =
            serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn get_vote_check(&self, bot_id: &str, user_id: &str) -> Response<VoteCheck> {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, self.authorization.parse().unwrap());
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let mut query = HashMap::new();
        query.insert("userID", user_id.to_string());
        let mut url = String::from("bots/");
        url.push_str(bot_id);
        url.push_str("/vote");

        let fetch = Client::_fetch("GET", &url, Some(header), Some(query), None);

        let json: Response<VoteCheck> =
            serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn update(
        &self,
        bot_id: &str,
        servers: Option<usize>,
        shards: Option<usize>,
    ) -> ResponseUpdate {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, self.authorization.parse().unwrap());
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let mut json = HashMap::new();
        if let Some(servers) = servers {
            json.insert("servers", servers.to_string());
        } else if let Some(shards) = shards {
            json.insert("shards", shards.to_string());
        }

        let mut url = String::from("bots/");
        url.push_str(bot_id);
        url.push_str("/stats");

        let fetch = Client::_fetch("POST", &url, Some(header), None, Some(json));

        let json: ResponseUpdate =
            serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn get_user(&self, user_id: &str) -> Response<UserInfo> {
        let mut url = String::from("users/");
        url.push_str(user_id);

        let fetch = Client::_fetch("GET", &url, None, None, None);
        let json: Response<UserInfo> =
            serde_json::from_str(&fetch.await.text().await.unwrap()).unwrap();

        json
    }

    pub async fn get_bot_widget(
        &self,
        bot_id: &str,
        widget_type: WidgetType,
        widget_query: Option<WidgetQuery>,
    ) -> String {
        let query = {
            if let Some(widget_query) = widget_query {
                let mut query: HashMap<&str, String> = HashMap::new();
                query.insert("style", widget_query.to_style().to_string());
                query.insert("icon", widget_query.to_icon().to_string());
                query.insert("scale", widget_query.to_scale().to_string());

                Some(query)
            } else {
                None
            }
        };
        let mut url = String::from("widget/bots/");
        url.push_str(&widget_type.to_string());
        url.push('/');
        url.push_str(bot_id);
        url.push_str(".svg");

        let fetch = Client::_fetch("GET", &url, None, query, None);

        fetch.await.url().to_string()
    }

    async fn _fetch(
        method: &str,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<HashMap<&str, String>>,
        json: Option<HashMap<&str, String>>,
    ) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = {
            let path_url = format!("{}/{}", BASE_URL, path);
            if let Some(query) = query {
                Url::parse_with_params(&path_url, query)
                    .unwrap()
                    .to_string()
            } else {
                path_url
            }
        };

        match method {
            "GET" => {
                if let Some(headers) = headers {
                    if let Some(json) = json {
                        client
                            .get(url)
                            .headers(headers)
                            .json(&json)
                            .send()
                            .await
                            .unwrap()
                    } else {
                        client.get(url).headers(headers).send().await.unwrap()
                    }
                } else if let Some(json) = json {
                    client.get(url).json(&json).send().await.unwrap()
                } else {
                    client.get(url).send().await.unwrap()
                }
            }
            "POST" => {
                if let Some(headers) = headers {
                    if let Some(json) = json {
                        client
                            .post(url)
                            .headers(headers)
                            .json(&json)
                            .send()
                            .await
                            .unwrap()
                    } else {
                        client.post(url).headers(headers).send().await.unwrap()
                    }
                } else if let Some(json) = json {
                    client.post(url).json(&json).send().await.unwrap()
                } else {
                    client.post(url).send().await.unwrap()
                }
            }
            _ => panic!(""),
        }
    }
}
