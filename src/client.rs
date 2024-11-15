use crate::project::Project;
use crate::task::Task;
use crate::time_record::TimeRecord;
use crate::user::User;
use chrono::NaiveDate;
use reqwest::header::HeaderMap;
use reqwest::{Client as HTTPClient, RequestBuilder, Response, StatusCode};
use serde::ser::Serialize;
pub struct Client {
    client: HTTPClient,
}

type Query = Vec<(String, String)>;

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

    fn get_request_builder(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<Query>,
    ) -> RequestBuilder {
        let base_url = "https://api.everhour.com";
        let url = format!("{base_url}{path}");
        let mut request_builder = self.client.request(method, url);
        if let Some(query) = query {
            request_builder = request_builder.query(&query);
        }
        request_builder
    }

    async fn get(&self, path: &str, query: Option<Query>) -> Result<Response, String> {
        let request = self
            .get_request_builder(reqwest::Method::GET, path, query)
            .build()
            .unwrap();
        println!("GET {}", request.url());
        match self.client.execute(request).await {
            Ok(response) => match response.status() {
                StatusCode::OK => Err(format!("Got status code {}", response.status())),
                _ => Ok(response),
            },
            Err(error) => Err(format!("{error}")),
        }
    }
    async fn post<T: Serialize>(&self, path: &str, data: T) -> Result<Response, String> {
        let request = self
            .get_request_builder(reqwest::Method::POST, path, None)
            .header("Content-Type", "application/json")
            .json::<T>(&data)
            .build()
            .unwrap();
        println!("POST {}", request.url());
        match self.client.execute(request).await {
            Ok(response) => Ok(response),
            Err(error) => Err(format!("{error}")),
        }
    }

    async fn put<T: Serialize>(&self, path: &str, data: T) -> Result<Response, String> {
        let request = self
            .get_request_builder(reqwest::Method::PUT, path, None)
            .header("Content-Type", "application/json")
            .json::<T>(&data)
            .build()
            .unwrap();
        println!("PUT {}", request.url());
        match self.client.execute(request).await {
            Ok(response) => Ok(response),
            Err(error) => Err(format!("{error}")),
        }
    }

    pub async fn get_current_user(&self) -> Result<User, String> {
        match self.get("/users/me", None).await {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }
    pub async fn get_user_time_records(
        &self,
        user_id: i64,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<TimeRecord>, String> {
        let mut query: Query = vec![];

        query.append(&mut vec![(String::from("limit"), String::from("100"))]);

        if let Some(from) = from {
            query.append(&mut vec![(String::from("from"), from.to_string())])
        }

        if let Some(to) = to {
            query.append(&mut vec![(String::from("to"), to.to_string())])
        }

        match self
            .get(format!("/users/{user_id}/time").as_str(), Some(query))
            .await
        {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }

    pub async fn get_task_time_records(
        &self,
        task_id: String,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<TimeRecord>, String> {
        let mut query: Query = vec![];

        query.append(&mut vec![(String::from("limit"), String::from("100"))]);

        if let Some(from) = from {
            query.append(&mut vec![(String::from("from"), from.to_string())])
        }

        if let Some(to) = to {
            query.append(&mut vec![(String::from("to"), to.to_string())])
        }

        match self
            .get(format!("/tasks/{task_id}/time").as_str(), Some(query))
            .await
        {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }

    pub async fn get_projects(&self, search_query: Option<String>) -> Result<Vec<Project>, String> {
        let mut query: Query = vec![];

        query.append(&mut vec![(String::from("limit"), String::from("100"))]);

        if let Some(search_query) = search_query {
            query.append(&mut vec![(String::from("query"), search_query.to_string())])
        }

        match self.get(format!("/projects").as_str(), Some(query)).await {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }
    pub async fn get_project_tasks(
        &self,
        project_id: String,
        search_query: Option<String>,
        exclude_closed: Option<bool>,
    ) -> Result<Vec<Task>, String> {
        let mut query: Query = vec![];

        query.append(&mut vec![(String::from("limit"), String::from("100"))]);

        if let Some(search_query) = search_query {
            query.append(&mut vec![(String::from("query"), search_query.to_string())])
        }

        if let Some(exclude_closed) = exclude_closed {
            query.append(&mut vec![(
                String::from("excludeClosed"),
                exclude_closed.to_string(),
            )])
        }

        match self
            .get(
                format!("/projects/{project_id}/tasks").as_str(),
                Some(query),
            )
            .await
        {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }

    pub async fn add_task_time_record(
        &self,
        task_id: String,
        time_record: TimeRecord,
    ) -> Result<(), String> {
        let response = self
            .post(&format!("/tasks/{task_id}/time"), time_record)
            .await
            .unwrap();
        if response.status() != 201 {
            Err(format!("{:?}", response.text().await.unwrap()))
        } else {
            Ok(())
        }
    }
    pub async fn update_task_time_record(
        &self,
        task_id: String,
        time_record: TimeRecord,
    ) -> Result<(), String> {
        let response = self
            .put(&format!("/tasks/{task_id}/time"), time_record)
            .await
            .unwrap();
        if response.status() != 200 {
            Err(format!("{:?}", response.text().await.unwrap()))
        } else {
            Ok(())
        }
    }
}
