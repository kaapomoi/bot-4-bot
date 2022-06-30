pub mod fetcher {
    extern crate reqwest;
    use reqwest::Client;
    use std::env;

    pub async fn fetch_summoner_id(summoner_name: &str) -> String {
        let token = env::var("RIOT_API_TOKEN").expect("Expected a token in the environment");
        let mut uri =
            "https://euw1.api.riotgames.com/tft/summoner/v1/summoners/by-name/".to_string();
        uri.push_str(summoner_name);
        let client = Client::new();

        let response = client
            .get(uri)
            .header("Accept-Language", "en-GB,en;q=0.5")
            .header(
                "Accept-Charset",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("X-Riot-Token", &token)
            .send()
            .await;

        let resp = response.ok();

        resp.unwrap().text().await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::fetcher::fetch_summoner_id;
}
