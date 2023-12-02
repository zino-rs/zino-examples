use kip_sql::db::Database;
use kip_sql::implement_from_tuple;
use kip_sql::storage::kip::KipStorage;
use kip_sql::types::LogicalType;
use kip_sql::types::value::DataValue;
use kip_sql::types::tuple::Tuple;
use serde::{Deserialize, Serialize};
use zino_core::model::{Model, ModelHooks};
use crate::model::into_insert_sql;

/// The `User` model.
#[derive(
Debug,
Clone,
Default,
Serialize,
Deserialize,
)]
#[serde(default)]
pub struct User {
    // Basic fields.
    pub id: i64,
    pub name: String,
    pub status: String,
    pub description: String,

    // Revisions.
    // TODO
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
    // pub version: u64,
}

impl Model for User {}

impl ModelHooks for User {}

impl User {
    pub(crate) async fn insert(&self, db: &Database<KipStorage>) {
        let sql = into_insert_sql(
            "users",
            vec![
                ("id", self.id.to_string()),
                ("name", self.name.clone()),
                ("status", self.status.clone()),
                ("description", self.description.clone()),
                // "created_at",
                // "updated_at",
                // "version"
            ]
        );

        let _ = db.run(&sql).await.unwrap();
    }

    pub(crate) async fn fetch_by_id(id: i64, db: &Database<KipStorage>) -> Option<User> {
        let mut tuple = db.run(&format!("select * from users where id = {}", id))
            .await
            .unwrap();

        if tuple.is_empty() {
            None
        } else {
            Some(User::from(tuple.remove(0)))
        }
    }
}

implement_from_tuple!(
    User, (
        id: i64 => |inner: &mut User, value: DataValue| {
            if let Some(val) = value.i64() {
                inner.id = val;
            }
        },
        name: String => |inner: &mut User, value: DataValue| {
            if let Some(val) = value.utf8() {
                inner.name = val;
            }
        },
        status: String => |inner: &mut User, value: DataValue| {
            if let Some(val) = value.utf8() {
                inner.status = val;
            }
        },
        description: String => |inner: &mut User, value: DataValue| {
            if let Some(val) = value.utf8() {
                inner.description = val;
            }
        }

        // created_at: DateTime<Utc> => |inner: &mut User, value: DataValue| {
        //     if let Some(val) = value.datetime() {
        //         inner.created_at = val.and_utc();
        //     }
        // },
        // updated_at: DateTime<Utc> => |inner: &mut User, value: DataValue| {
        //     if let Some(val) = value.datetime() {
        //         inner.updated_at = val.and_utc();
        //     }
        // },
        // version: u64 => |inner: &mut User, value: DataValue| {
        //     if let Some(val) = value.u64() {
        //         inner.version = val;
        //     }
        // }
    )
);