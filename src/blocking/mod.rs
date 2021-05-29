use std::collections::HashMap;

use reqwest::{
    header::HeaderMap,
    header::{AUTHORIZATION, CONTENT_TYPE},
    Url,
};

use crate::{BASE_URL, model::{Bot, UserInfo, VoteCheck, WidgetQuery, WidgetType, response::{Data, Response, ResponseUpdate}}};

pub struct Client {
    authorization: &'static str,
}

impl Client {
    pub fn new(authorization: &'static str) -> Self {
        Self { authorization }
    }

    pub fn get_bot(&self, bot_id: &str) -> Response<Bot> {
        let fetch = Client::_fetch("GET", format!("bots/{}", bot_id).as_str(), None, None, None);
        let json: Response<Bot> = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn search_bot(&self, search: &str, page: Option<usize>) -> Response<Data<Bot>> {
        let mut query = HashMap::new();
        query.insert("query", search.to_string());
        if let Some(page) = page {
            query.insert("page", page.to_string());
        } else {
            query.insert("page", "1".to_string());
        }
        let fetch = Client::_fetch("GET", "search/bots", None, Some(query), None);

        let json: Response<Data<Bot>> = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn get_votes_list(&self, page: Option<usize>) -> Response<Data<Bot>> {
        let mut query = HashMap::new();
        if let Some(page) = page {
            query.insert("page", page.to_string());
        } else {
            query.insert("page", "1".to_string());
        }
        let fetch = Client::_fetch("GET", "list/bots/votes", None, Some(query), None);

        let json: Response<Data<Bot>> = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn get_new_bot_list(&self) -> Response<Data<Bot>> {
        let fetch = Client::_fetch("GET", "list/bots/new", None, None, None);

        let json: Response<Data<Bot>> = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn get_vote_check(&self, bot_id: &str, user_id: &str) -> Response<VoteCheck> {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, self.authorization.parse().unwrap());
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let mut query = HashMap::new();
        query.insert("userID", user_id.to_string());
        let fetch = Client::_fetch(
            "GET",
            format!("bots/{}/vote", bot_id).as_str(),
            Some(header),
            Some(query),
            None,
        );

        let json: Response<VoteCheck> = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn update_servers(&self, bot_id: &str, servers: usize) -> ResponseUpdate {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, self.authorization.parse().unwrap());
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let mut json = HashMap::new();
        json.insert("servers", servers.to_string());

        let fetch = Client::_fetch(
            "POST",
            format!("bots/{}/stats", bot_id).as_str(),
            Some(header),
            None,
            Some(json),
        );

        let json: ResponseUpdate = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn get_user(&self, user_id: &str) -> Response<UserInfo> {
        let fetch = Client::_fetch(
            "GET",
            format!("users/{}", user_id).as_str(),
            None,
            None,
            None,
        );
        let json: Response<UserInfo> = serde_json::from_str(&fetch.text().unwrap()).unwrap();

        json
    }

    pub fn get_bot_widget(
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

        let fetch = Client::_fetch(
            "GET",
            format!("widget/bots/{}/{}.svg", widget_type.to_string(), bot_id).as_str(),
            None,
            query,
            None,
        );

        fetch.url().to_string()
    }

    fn _fetch(
        method: &str,
        path: &str,
        headers: Option<HeaderMap>,
        query: Option<HashMap<&str, String>>,
        json: Option<HashMap<&str, String>>,
    ) -> reqwest::blocking::Response {
        let client = reqwest::blocking::Client::new();
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
                        client.get(url).headers(headers).json(&json).send().unwrap()
                    } else {
                        client.get(url).headers(headers).send().unwrap()
                    }
                } else if let Some(json) = json {
                    client.get(url).json(&json).send().unwrap()
                } else {
                    client.get(url).send().unwrap()
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
                            .unwrap()
                    } else {
                        client.post(url).headers(headers).send().unwrap()
                    }
                } else if let Some(json) = json {
                    client.post(url).json(&json).send().unwrap()
                } else {
                    client.post(url).send().unwrap()
                }
            }
            _ => panic!(""),
        }
    }
}
