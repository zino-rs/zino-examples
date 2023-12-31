use crate::extension::rbatis;
use rbs::value::Value;
use std::time::Instant;
use zino::{prelude::*, Request, Response, Result};
use zino_model::{Tag, User};

pub async fn rbatis_user_view(req: Request) -> Result {
    let user_id = req.parse_param::<i64>("id")?;

    let db_query_start_time = Instant::now();
    let table_name = User::table_name();
    let args = vec![Value::String(user_id.to_string())];
    let sql = format!("SELECT * FROM {table_name} WHERE id = ?;");
    let user = rbatis::RBATIS.query(&sql, args).await.extract(&req)?;
    let db_query_duration = db_query_start_time.elapsed();

    let data = json!({
        "entry": user,
    });
    let mut res = Response::default().context(&req);
    res.record_server_timing("db", None, Some(db_query_duration));
    res.set_data(&data);
    Ok(res.into())
}

pub async fn rbatis_tag_view(req: Request) -> Result {
    let tag_id = req.parse_param::<i64>("id")?;

    let db_query_start_time = Instant::now();
    let table_name = Tag::table_name();
    let args = vec![Value::String(tag_id.to_string())];
    let sql = format!("SELECT * FROM {table_name} WHERE id = ?;");
    let tag = rbatis::RBATIS.query(&sql, args).await.extract(&req)?;
    let db_query_duration = db_query_start_time.elapsed();

    let data = json!({
        "entry": tag,
    });
    let mut res = Response::default().context(&req);
    res.record_server_timing("db", None, Some(db_query_duration));
    res.set_data(&data);
    Ok(res.into())
}
