//! A Rust Asana SDK, for the Asana Rest API
//!
//! This crate uses a model! macro to define flexible Model Structs with a very lean syntax.
//! These generated structs are used to deserialize entities, select include fields and embed relationships from the Asana API.
//!
//! The Asana API returns flexible objects with varying field & relation includes, so this crate uses models provided by the user.
//! This makes the crate also compatible with entities added to the API by Asana in the future.
//!
//! To make the interface as ergonomic as possible, it relies on two components:
//!
//! * A `model!()` macro to easily define deserialization Structs ([serde](https://docs.rs/serde/)), together with endpoint urls and field/relation inclusion querystrings.
//! * Turbofish operators (`get::<Type>()`) to make API calls for defined models.
//!
//! ## Sample usage
//!
//! ```
//! use reqwest::{Error};
//! use asana_sdk::*;
//! use asana_sdk::models::Model;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!
//!     // Connect with your Asana PAT (token), from https://app.asana.com/0/developer-console
//!     let mut asana = Asana::connect(String::from("1/your:personal-access-token"));
//!
//!     // A Model Struct linked to the "users" endpoint
//!     model!(User "users" {
//!         email: String,
//!         name: String,
//!     });
//!
//!     // Simple calls to get one or multiple users
//!     let mut user:  User      = asana.get::<User>("me").await;
//!     let mut users: Vec<User> = asana.list::<User>().await;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### A few more advanced examples:
//!
//! Compound call to list all sections *within* a specific project
//! ```
//! model!(Section "sections" { name: String });
//! model!(Project "projects" { name: String });
//!
//! let mut sections = asana
//!     .from::<Project>("12345678")
//!     .list::<Section>().await;
//! ```
//!
//! A Struct for Tasks including Projects.
//! TaskWithProjects is just an example name, you can give the Struct any name you want.
//!
//! The call will list all tasks from a specific section,
//! and include all other projects the task is part of.
//! ```
//! model!(TaskWithProjects "tasks" {
//!     name: String,
//!     projects: Vec<Project>
//! } Project);
//!
//! let mut tasks_with_projects = asana
//!      .from::<Section>("12345678")
//!      .list::<TaskWithProjects>().await;
//! ```
//!
//! Note that all model Structs by default include gid & resource_type,
//! So it's not mandatory to include other fields.
//!
//! Fields which might be null in the API should be deserialized into an Option<Type>
//! ```
//! model!(Assignee "assignee" {});
//! model!(TaskWithAssignee "tasks" {
//!     name: String,
//!     assignee: Option<Assignee>
//! } Assignee);
//! ```

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
                .user_agent("asana_sdk.rs/0.1.2")
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