use std::fmt;

use serde::{Deserialize, Serialize};
pub mod response;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub flags: usize,
    pub github: Option<String>,
    pub tag: String,
    pub username: String,
    pub bots: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub id: String,
    pub flags: usize,
    pub name: String,
    pub tag: String,
    pub avatar: Option<String>,
    pub owners: Vec<User>,
    pub lib: String,
    pub prefix: String,
    pub votes: usize,
    pub servers: usize,
    pub intro: String,
    pub desc: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub web: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub git: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub discord: String,
    pub category: Vec<String>,
    pub vanity: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub banner: String,
    pub status: Option<Status>,
    pub state: State,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub flags: usize,
    pub github: Option<String>,
    pub tag: String,
    pub username: String,
    pub bots: Vec<UserInfoBot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoBot {
    pub id: String,
    pub flags: usize,
    pub name: String,
    pub tag: String,
    pub avatar: Option<String>,
    pub owners: Vec<String>,
    pub lib: String,
    pub prefix: String,
    pub votes: usize,
    pub servers: usize,
    pub intro: String,
    pub desc: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub web: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub git: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub discord: String,
    pub category: Vec<String>,
    pub vanity: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub banner: String,
    pub status: Option<Status>,
    pub state: State,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteCheck {
    pub voted: bool,
    #[serde(rename = "lastVote")]
    pub last_vote: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserFlags {
    None = 0,
    Admin = 1 << 0,
    BugHunter = 1 << 1,
    BotReviewer = 1 << 2,
    PremiumUser = 1 << 3,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotFlags {
    None = 0,
    OfficialBot = 1 << 0,
    KoreaDiscordBotListCertifiedBot = 1 << 2,
    Partner = 1 << 3,
    DiscordVerifiedBot = 1 << 4,
    Premium = 1 << 5,
    OnestKoreaDiscordBotListHackathonWinnerBot = 1 << 6,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Online,
    Idle,
    Dnd,
    Streaming,
    Offline,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Ok,
    Reported,
    Blocked,
    Private,
    Archived,
}

pub enum WidgetType {
    Votes,
    Servers,
    Status,
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WidgetType::Servers => write!(f, "servers"),
            WidgetType::Votes => write!(f, "votes"),
            WidgetType::Status => write!(f, "status"),
        }
    }
}

#[derive(Clone)]
pub struct WidgetQuery {
    style: Option<&'static str>,
    scale: Option<f32>,
    icon: Option<bool>,
}

impl WidgetQuery {
    pub fn to_style(&self) -> &str {
        match self.style {
            Some("classic") => "classic",
            Some("flat") => "flat",
            None => "flat",
            _ => "flat",
        }
    }
    pub fn to_scale(&self) -> f32 {
        if Some(0.5) <= self.scale && self.scale <= Some(3.0) {
            self.scale.unwrap()
        } else {
            1.0
        }
    }
    pub fn to_icon(&self) -> &str {
        match self.icon {
            Some(true) => "true",
            Some(false) => "false",
            None => "true",
        }
    }
}
