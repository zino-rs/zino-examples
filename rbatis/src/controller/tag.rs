use crate::model::Tag;
use std::time::Instant;
use zino::{prelude::*, Request, Response, Result};

pub async fn view(req: Request) -> Result {
    let tag_id = req.parse_param("id")?;

    let db_query_start_time = Instant::now();
    let tag = Tag::fetch_by_id(&tag_id).await.extract(&req)?;
    let db_query_duration = db_query_start_time.elapsed();

    let data = Map::data_entry(tag);
    let mut res = Response::default().context(&req);
    res.record_server_timing("db", None, Some(db_query_duration));
    res.set_data(&data);
    Ok(res.into())
}
