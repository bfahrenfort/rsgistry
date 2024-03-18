// Macro uses an all-caps prefix to be visually distinct
#![allow(non_snake_case)]

use macros::{Bindable, Countable, FromQueue, Keyed, Listable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{database::HasArguments, query::QueryAs, FromRow, Postgres};

// This is exactly how your Entry looks in the database migrations, less the ID
// - Use Option<SomeType> for fields that can be NULL
// - Others will be NOT NULL
// You can probably use this type to send your requests from your custom API consumers as well!
// - ...obviously minus the mixin and custom macro stuff
// Make sure one field is prepended with UNIQUE, this will be used for fetches
// - ex. a package or extension name
#[derive(Serialize, Deserialize, JsonSchema, Countable, Listable, Keyed, FromQueue, Bindable)]
#[bind_to(EntryWithID)]
#[mixin::declare]
pub struct Entry {
    #[serde(rename = "name")] // Allows the database to work without the UNIQUE_ qualifier
    pub UNIQUE_name: String,
    // pub doctype: String,
    // pub url: Option<String>,
}

#[mixin::insert(Entry)]
#[derive(Deserialize, JsonSchema, FromRow, Serialize, Countable, Listable, Bindable)]
#[bind_to(Queue)]
pub struct QueueNew {
    pub request_type: String,
}

// Internal database types, basically all of the above Entry plus the automatically-generated fields
// You may need to adjust these depending on your schema, but it's not likely
#[mixin::insert(Entry)]
#[derive(Serialize, JsonSchema, FromRow, Listable)]
pub struct EntryWithID {
    pub id: i32,
}

#[mixin::insert(Entry)]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, FromRow, Listable)]
pub struct Queue {
    pub id: i32,
    pub request_type: String,
}
