use macros::{Countable, Listable};
use serde::{Deserialize, Serialize};
use sqlx::{database::HasArguments, query::QueryAs, FromRow, Postgres};

// This is exactly how your Entry looks in the database migrations, less the ID
// - Use Option<SomeType> for fields that can be NULL
// - Others will be NOT NULL
// You can probably use this type to send your requests from your API consumers as well!
// - ...obviously minus the mixin junk
#[derive(Serialize, Deserialize, Debug, Countable, Listable)]
#[mixin::declare]
pub struct Entry {
    pub program_name: String,
    pub doctype: String,
    pub url: Option<String>,
}

impl Entry {
    pub fn bind<'q>(
        self: &'q Entry,
        query: QueryAs<'q, Postgres, EntryWithID, <Postgres as HasArguments>::Arguments>,
    ) -> QueryAs<'q, Postgres, EntryWithID, <Postgres as HasArguments>::Arguments> {
        query
            .bind(&self.program_name)
            .bind(&self.doctype)
            .bind(&self.url)
    }
}

#[mixin::insert(Entry)]
#[derive(Deserialize, FromRow, Serialize, Countable, Listable)]
pub struct QueueNew {
    pub request_type: String,
}

impl QueueNew {
    pub fn bind<'q>(
        self: &'q QueueNew,
        query: QueryAs<'q, Postgres, Queue, <Postgres as HasArguments>::Arguments>,
    ) -> QueryAs<'q, Postgres, Queue, <Postgres as HasArguments>::Arguments> {
        query
            .bind(&self.program_name)
            .bind(&self.doctype)
            .bind(&self.url)
            .bind(&self.request_type)
    }
}
// Internal database types, basically all of the above Entry plus the automatically-generated fields
// You may need to adjust these depending on your schema, but it's not likely
#[mixin::insert(Entry)]
#[derive(Serialize, FromRow, Listable)]
pub struct EntryWithID {
    pub id: i32,
}

#[mixin::insert(Entry)]
#[derive(Serialize, Deserialize, Debug, Clone, FromRow, Listable)]
pub struct Queue {
    pub id: i32,
    pub request_type: String,
}
