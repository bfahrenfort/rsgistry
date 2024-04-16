use aide::OperationIo;
use macros::{Countable, FetchBindable, FromQueue, Keyed, Listable, PushBindable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{database::HasArguments, query::QueryAs, FromRow, Postgres};

// This is exactly how your Entry looks in the database migrations, less the ID
// - Use Option<SomeType> for fields that can be NULL
// - Others will be NOT NULL
// You can probably use this type to send your requests from your custom API consumers as well!
// - ...obviously minus the mixin and custom macro stuff
// Make sure you specify at least one field (multiple supported, comma-separated)
//   in the keys attribute, these should be your database keys
//   (specified as UNIQUE (key1, key2) in migrations)
#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Countable,
    Listable,
    Keyed,
    FromQueue,
    PushBindable,
    FetchBindable,
    OperationIo,
)]
#[bind_to(EntryWithID)]
#[keys(name)]
#[mixin::declare]
pub struct Entry {
    pub name: String,
    // pub doctype: String,
    // pub url: Option<String>,
}

// ***
// Everything else in this file is only here for scoping reasons.
// You can safely ignore for 99% of use cases.
// ***

// Internal database types, basically all of the above Entry plus queue fields
// You may need to adjust these depending on your schema, but it's not likely
#[mixin::insert(Entry)]
#[derive(
    Deserialize, OperationIo, JsonSchema, FromRow, Serialize, Countable, Listable, PushBindable,
)]
#[bind_to(Queue)]
pub struct QueueNew {
    pub request_type: String,
}

// The full main database type
// If you add any extra administrative fields, track them here
#[mixin::insert(Entry)]
#[derive(Serialize, OperationIo, JsonSchema, FromRow, Listable)]
pub struct EntryWithID {
    pub id: i32,
}

#[mixin::insert(Entry)]
#[derive(Serialize, Deserialize, OperationIo, JsonSchema, Debug, Clone, FromRow, Listable)]
pub struct Queue {
    pub id: i32,
    pub request_type: String,
}
