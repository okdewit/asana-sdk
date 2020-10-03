use reqwest::{Method, Response};
use std::vec::Vec;
use log::*;

pub mod models;
use crate::models::*;

pub struct Asana;

const API_VERSION: &str = "1.0";

pub struct Client {
    client: reqwest::Client,
    token: String,
    endpoint: String,
}

impl Asana {
    pub fn connect(token: String) -> Client {
        Client {
            token,
            endpoint: String::from(""),
            client: reqwest::Client::builder()
                .user_agent("AsanaGraph/1.0.0")
                .build().unwrap(),
        }

    }
}

impl Client {
    pub async fn get<T: Model>(&mut self, gid: &str) -> T {
        let model: Wrapper<T> = self
            .call::<T>(Method::GET, Some(gid)).await
            .json().await.unwrap();

        model.data
    }

    pub async fn list<T: Model>(&mut self) -> Vec<T> {
        let model: ListWrapper<T> =  self
            .call::<T>(Method::GET, None).await
            .json().await.unwrap();

        self.endpoint.clear();

        model.data
    }

    pub fn from<T: Model>(&mut self, relational_gid: &str) -> &mut Client {
        self.endpoint = format!("{}/{}/", T::endpoint(), relational_gid);
        self
    }

    async fn call<T: Model>(&mut self, method: Method, gid: Option<&str>) -> Response {
        // Add both relational and main endpoints, and entity gid if supplied
        let url = format!("{}{}/", self.endpoint, T::endpoint());
        let url = format!("{}{}", url, match gid {
            Some(gid) => format!("{}", gid),
            None => "".to_string()
        });

        // Clear relational endpoint state from client
        self.endpoint.clear();

        // Add relational & root field inclusions as query parameters
        let opts = format!("this.({}),{}", T::field_names().join("|"), T::opt_strings().join(","));
        let url = format!("{}?opt_fields={}", url, opts);

        let request_url = format!("https://app.asana.com/api/{}/{}", API_VERSION, url);
        info!("{}", request_url);

        self.client.request(method, &request_url)
            .header("Authorization", format!("Bearer {}", &self.token))
            .send().await.unwrap()
    }
}