use serde::{Deserialize};
use serde::de::DeserializeOwned;

/// A macro for generating Asana model structs
///
/// # Examples
///
/// The example below will generate a `User` struct, linked to the `/users` Asana endpoint.
///
/// * It will include & deserialize the `email` & `name` fields as Strings.
/// * The fields `gid` and `resource_type` are included by default, and don't have to be specified.
///
/// Any extra fields returned from the Asana API are flattened by [`Serde`] into a [`Hashmap`]:
///
/// ```
/// use asana_sdk::models::Model;
///
/// model!(User "users" {
///     email: String,
///     name: String,
/// });
/// ```
#[macro_export]
macro_rules! model {
    ($name:ident $endpoint:literal { $( $field:ident: $fty:ty ),* $(,)? } $( $include:ident),* $(,)?) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct $name {
            gid: String,
            resource_type: String,
            $( $field: $fty, )*
            #[serde(flatten)]
            extra: std::collections::HashMap<String, serde_json::Value>,
        }

        impl Model for $name {
            fn endpoint() -> String { $endpoint.to_string() }

            fn opt_strings() -> Vec<String> {
                vec![$(format!("{}.({})", $include::endpoint(), $include::field_names().join("|"))),*]
            }

            fn field_names() -> &'static [&'static str] {
                &["resource_type", $(stringify!($field)),*]
            }
        }
    };
}

pub trait Model: DeserializeOwned {
    fn endpoint() -> String;
    fn field_names() -> &'static [&'static str];
    fn opt_strings() -> Vec<String>;
}

#[derive(Deserialize, Debug)]
pub(crate) struct Wrapper<T> {
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ListWrapper<T> {
    pub data: Vec<T>,
}