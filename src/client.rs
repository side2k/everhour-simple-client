use crate::time_record::TimeRecord;
use crate::user::User;
use reqwest::header::HeaderMap;
use reqwest::{Client as HTTPClient, Response, StatusCode};
pub struct Client {
    client: HTTPClient,
}

impl Client {
    pub fn new(api_token: String) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("X-Api-Key", api_token.parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .unwrap();
        Client { client }
    }

    async fn get(&self, path: &str) -> Result<Response, String> {
        let base_url = "https://api.everhour.com";
        let url = format!("{base_url}{path}");
        let request = self
            .client
            .request(reqwest::Method::GET, url)
            .build()
            .unwrap();
        match self.client.execute(request).await {
            Ok(response) => {
                if response.status() != StatusCode::OK {
                    panic!("Got status code {}", response.status());
                }
                Ok(response)
            }
            Err(error) => Err(format!("{error}")),
        }
    }

    pub async fn get_current_user(&self) -> Result<User, String> {
        match self.get("/users/me").await {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }
    pub async fn get_user_time_records(&self, user_id: i64) -> Result<Vec<TimeRecord>, String> {
        match self
            .get(format!("/users/{user_id}/time?limit=1").as_str())
            .await
        {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }
}
