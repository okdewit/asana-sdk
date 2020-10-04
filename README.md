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

Included fields/relationships, and relationship constraints are also possible.  
You only need to define the fields you actually need on each model.
```Rust
model!(Tasks "tasks" {
    name: String,
    projects: Vec<Project>
    assignee: Option<Assignee>
} Project, Assignee);

model!(Project "projects" { name: String });
model!(Assignee "assignee" { name: String });
model!(Section "sections" {});

let mut tasks_with_projects_and_assignee = asana
     .from::<Section>("12345678")
     .list::<Tasks>().await;
```

## Contributions

This crate is far from perfect, so contributions are very welcome!  
It still needs tests and support for persisting models through `POST`/`PUT` requests, for example.
 
Just open an [issue](https://github.com/okdewit/asana-sdk/issues) or make a [pull request](https://github.com/okdewit/asana-sdk/pulls).