# asana-sdk

A Rust Asana SDK, for the Asana Rest API

This crate uses a `model!` macro to define flexible Model Structs with a very lean syntax.   
These generated structs are used to deserialize entities, select include fields and embed relationships from the Asana API.

See https://docs.rs/asana-sdk for complete documentation.

```Rust
model!(User "users" {
    email: String,
    name: String,
});

// Simple calls to get one or multiple users
let mut user:  User      = asana.get::<User>("me").await;
let mut users: Vec<User> = asana.list::<User>().await;
```

```Rust
model!(TaskWithProjects "tasks" {
    name: String,
    projects: Vec<Project>
} Project);

let mut tasks_with_projects = asana
     .from::<Section>("12345678")
     .list::<TaskWithProjects>().await;
```
