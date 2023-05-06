use std::collections::HashMap;
use reqwest::{Client, Response};

use std::env;

pub struct ApiClient {
    token: String
}

const API_VERSION: &str = "2022-06-28";
const BASE_URL: &str = "https://api.notion.com";

impl ApiClient {
    pub async fn get(self, path: &str, params: Option<&[(&str, &str)]>) -> Result<Response, Box<dyn std::error::Error>> {
        let url = reqwest::Url::parse_with_params(&ApiClient::build_request_url(path), params.unwrap_or(&[]))?;
        let client = ApiClient::client();
        let response = client.get(url)
            .bearer_auth(self.token)
            .header("Notion-Version", API_VERSION)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn post(self, path: &str, body: String) -> Result<Response, Box<dyn std::error::Error>> {
        let url = reqwest::Url::parse(&ApiClient::build_request_url(path))?;
        let client = ApiClient::client();
        let response = client.post(url)
            .bearer_auth(self.token)
            .header("Notion-Version", API_VERSION)
            .json(&body)
            .send()
            .await?;
        Ok(response)
    }

    fn build_request_url(path: &str) -> String {
        let mut url = BASE_URL.to_string();
        url.push_str(&path);

        url
    }

    fn client() -> Client {
        reqwest::Client::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_request_url() {
        let path = "/v1/users";
        let expected = "https://api.notion.com/v1/users";

        assert_eq!(ApiClient::build_request_url(path), expected);
    }
}
