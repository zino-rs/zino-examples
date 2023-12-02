use std::sync::OnceLock;
use std::time::Instant;
use kip_sql::db::Database;
use kip_sql::storage::kip::KipStorage;
use zino::{prelude::*, Request, Response, Result};
use crate::model::User;

pub static INSTANCE: OnceLock<Database<KipStorage>> = OnceLock::new();

pub fn kip_sql() -> &'static Database<KipStorage> {
    INSTANCE.get().unwrap()
}
pub async fn kip_sql_user_view(req: Request) -> Result {
    let user_id = req.parse_param::<i64>("id")?;

    let db_query_start_time = Instant::now();
    let sql = format!("SELECT * FROM users WHERE id = {};", user_id);

    let mut tuples = kip_sql().run(&sql).await.unwrap();
    let db_query_duration = db_query_start_time.elapsed();
    let data = json!({
        "entry": User::from(tuples.remove(0)),
    });
    let mut res = Response::default().context(&req);

    res.record_server_timing("db", None, Some(db_query_duration));
    res.set_data(&data);
    Ok(res.into())
}
