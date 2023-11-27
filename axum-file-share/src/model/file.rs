use serde::{Deserialize, Serialize};
use std::time::Duration;
use zino::prelude::*;
use zino_core::{Map, Uuid};
use zino_derive::{DecodeRow, Model, ModelAccessor, ModelHooks, Schema};

/// The `file` model.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    Schema,
    ModelAccessor,
    ModelHooks,
    Model,
    DecodeRow,
)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct File {
    // Basic fields.
    #[schema(primary_key, read_only)]
    id: Uuid,
    #[schema(not_null, index_type = "text")]
    name: String,
    #[schema(unique, index_type = "text", comment = "file shortcode ID")]
    shortcode: String,
    #[schema(unique, index_type = "text", comment = "file md5")]
    md5: String,
    #[schema(default_value = "active", index_type = "hash")]
    status: String,
    #[schema(index_type = "text")]
    localurl: String,
    #[schema(index_type = "text")]
    localpath: String,
    #[schema(comment = "file expire time minute")]
    expire_time: u64,

    // Revisions.
    #[schema(readonly, default_value = "now", index_type = "btree")]
    created_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    updated_at: DateTime,
    #[schema(readonly, default_value = "now", index_type = "btree")]
    delete_at: DateTime,
}

impl File {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Self::default()
        }
    }
    pub fn set_short_code(&mut self, shortcode: &str) {
        self.shortcode = shortcode.to_owned();
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
    pub fn set_md5(&mut self, md5: &str) {
        self.md5 = md5.to_owned();
    }
    pub fn set_local_url(&mut self, local_url: &str) {
        self.localurl = local_url.to_owned();
    }
    pub fn set_local_path(&mut self, local_path: &str) {
        self.localpath = local_path.to_owned();
    }
    pub fn set_delete_time(&mut self, expirse_time_second: u64) {
        self.delete_at = DateTime::now() + Duration::from_secs(expirse_time_second);
    }
    pub fn set_unactive(&mut self) {
        self.status = "unactive".to_string();
    }
    pub fn get_name(&mut self) -> &str {
        &self.name
    }
    pub fn get_short_code(&mut self) -> &str {
        &self.shortcode
    }
    pub fn get_localpath(&mut self) -> &str {
        &self.localpath
    }
    pub fn get_localurl(&mut self) -> &str {
        &self.localurl
    }
}
