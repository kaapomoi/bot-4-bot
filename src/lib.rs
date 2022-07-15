pub mod fetcher {
    extern crate reqwest;
    use reqwest::{Client, Response};
    use serenity::{json::{JsonMap, json, self}, futures::future::ok};
    use std::{env::{self, VarError}, fmt::Debug, fs::File};

    async fn get_request(uri: String) -> Result<Response, reqwest::Error> {
        let token = env::var("RIOT_API_TOKEN").expect("Expected a token in the environment");
        let client = Client::new();

        client
            .get(uri)
            .header("Accept-Language", "en-GB,en;q=0.5")
            .header(
                "Accept-Charset",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("X-Riot-Token", &token)
            .send()
            .await
    }

    pub async fn fetch_summoner_id(summoner_name: &str) -> JsonMap {
        let uri = format!("https://euw1.api.riotgames.com/tft/summoner/v1/summoners/by-name/{}", summoner_name);
        

        let resp = get_request(uri).await.expect("Failed response");

        resp.json().await.unwrap()
    }

    pub async fn fetch_summoner_match_ids(summoner_name: &str, count: u16) -> String {
        let summoner_id_data = fetch_summoner_id(summoner_name).await;

        let puuid = summoner_id_data.get("puuid").unwrap().to_string();
        let uri = format!("https://europe.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?start=0&count={}", &puuid.trim_matches('"'), &count);

        let resp = get_request(uri).await.ok();
        
        resp.unwrap().text().await.unwrap()
    }

}

#[cfg(test)]
mod tests {
    use crate::fetcher::fetch_summoner_id;
}
