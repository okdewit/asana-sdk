use serde::{Deserialize};
use serde::de::DeserializeOwned;

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