use reqwest;
use std::collections::HashMap;

pub struct UserGithubInfo {
    pub avatar_url: String,
    pub bio: String,
    pub read_me: String,
}

impl UserGithubInfo {
    pub fn new(username: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("WeirdOne")
            .build()
            .unwrap();
        println!("Getting user data from GitHub...");
        let res = client
            .get(&format!("https://api.github.com/users/{username}"))
            .send()
            .expect("couldnt get user data")
            .text()
            .expect("no text in user data");

        let info: HashMap<String, serde_json::Value> = serde_json::from_str(&res).unwrap();
        let avatar_url = info
            .get("avatar_url")
            .expect("no avatar_url")
            .to_string()
            .replace("\"", "");
        let bio = info
            .get("bio")
            .expect("no bio")
            .to_string()
            .replace("\"", "");
        println!("Scraping README.md from GitHub...");
        let mut read_me = client
            .get(format!(
                "https://raw.githubusercontent.com/{}/{}/main/README.md",
                username, username
            ))
            .send()
            .expect("no readme file. is your readme file public?")
            .text()
            .unwrap();
        read_me.remove(read_me.len() - 1);
        read_me.remove(0);
        Self {
            avatar_url,
            bio,
            read_me,
        }
    }
}
